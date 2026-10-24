// File: fdtd_bus/src/thin_wire.rs
// Thin-wire subgridding for 6502 bus traces.

use nalgebra::{Vector3, DVector, DMatrix};
use ndarray::Array1;
use crate::fdtd::YeeGrid;

/// Thin wire segment (1D telegrapher's equation coupled to 3D FDTD)
#[derive(Debug)]
pub struct ThinWire {
    pub name: String,
    pub nodes: Vec<WireNode>, // Discretized along trace
    pub z0: f64, // Characteristic impedance
    pub v_phase: f64, // Phase velocity
    pub terminations: (Termination, Termination), // (near, far)
    // State variables
    pub v: DVector<f64>, // Voltage at nodes
    pub i: DVector<f64>, // Current at segments
    // Coupling
    pub e_coupling: Vec<CouplingPoint>, // Mapping to FDTD E-field nodes
}

#[derive(Debug, Clone)]
pub struct WireNode {
    pub position: Vector3<f64>,
    pub fdtd_indices: (usize, usize, usize), // Nearest E-field node
}

#[derive(Debug, Clone)]
pub struct CouplingPoint {
    pub wire_node_idx: usize,
    pub fdtd_ex_idx: (usize, usize, usize),
    pub fdtd_ey_idx: (usize, usize, usize),
    pub fdtd_ez_idx: (usize, usize, usize),
    pub weight: f64, // Interpolation weight
}

#[derive(Debug, Clone, Copy)]
pub enum Termination {
    Open,
    Short,
    Matched(f64), // Resistance
    PinModel { capacitance: f64, inductance: f64 }, // 6502 pin model
}

impl ThinWire {
    pub fn new_from_trace(trace: &crate::geometry::Trace, stackup: &crate::geometry::Stackup, dz: f64) -> Self {
        let length = (trace.end - trace.start).norm();
        let n_segments = (length / dz).ceil() as usize;
        let mut nodes = Vec::new();
        for k in 0..=n_segments {
            let t = k as f64 / n_segments as f64;
            let pos = trace.start + (trace.end - trace.start) * t;
            nodes.push(WireNode { position: pos.coords, fdtd_indices: (0,0,0) }); // Filled later
        }

        // Microstrip impedance approximation
        let w = trace.width;
        let h = stackup.fr4_thickness;
        let er = stackup.fr4_er;
        let z0 = 60.0 / (er.sqrt()) * ((8.0*h/w) + (w/(4.0*h))).ln();
        let v_phase = 299792458.0 / ((er + 1.0)/2.0).sqrt();

        let n = n_segments + 1;
        Self {
            name: trace.name.clone(),
            nodes,
            z0,
            v_phase,
            terminations: (Termination::PinModel { capacitance: 5e-12, inductance: 5e-9 }, Termination::Matched(z0)),
            v: DVector::zeros(n),
            i: DVector::zeros(n_segments),
            e_coupling: Vec::new(),
        }
    }

    /// Map wire nodes to FDTD E-field grid indices
    pub fn map_to_fdtd(&mut self, grid: &YeeGrid) {
        for (idx, node) in self.nodes.iter_mut().enumerate() {
            let i = (node.position.x / grid.dx).round() as usize;
            let j = (node.position.y / grid.dy).round() as usize;
            let k = (node.position.z / grid.dz).round() as usize;
            node.fdtd_indices = (i.clamp(0, grid.nx-1), j.clamp(0, grid.ny-1), k.clamp(0, grid.nz-1));
        }
        // Build coupling points (interpolate E-field to wire axis)
        // ...
    }

    /// Coupled update: FDTD E-field -> wire voltage, wire current -> FDTD H-field
    pub fn update_coupled(&mut self, grid: &mut YeeGrid) {
        let dt = grid.dt;
        // Segment spacing along the wire. (Was: z-difference only,
        // which is 0 for the canonical y-directed traces and made
        // every MNA coefficient divide by zero; the node distance is
        // the same quantity for z-aligned wires.)
        let dz = (self.nodes[1].position - self.nodes[0].position).norm();

        // 1. Compute incident E-field along wire from FDTD
        let mut e_inc = DVector::zeros(self.v.len());
        for (idx, node) in self.nodes.iter().enumerate() {
            let (i,j,k) = node.fdtd_indices;
            // Interpolate E-field components to wire tangent direction
            let ex = grid.ex[(i, j, k, 0)];
            let ey = grid.ey[(i, j, k, 1)];
            let ez = grid.ez[(i, j, k, 2)];
            e_inc[idx] = ez; // Assuming wire along z
        }

        // 2. Update telegrapher's equations (leapfrog)
        // V^{n+1} = V^n - (Z0 * dt / dz) * (I^{n+1/2} - I^{n-1/2}) + dt * e_inc
        // I^{n+1/2} = I^{n-1/2} - (dt / (Z0 * dz)) * (V^{n+1} - V^n)
        // With boundary conditions at terminations

        // Simplified: use MNA (Modified Nodal Analysis) for the wire network
        self.solve_mna(dt, dz, &e_inc);

        // 3. Back-couple: wire current -> equivalent H-field source in FDTD
        // J_eq = I * delta(r - r_wire) -> H-field update
        for seg_idx in 0..self.i.len() {
            // (Was: i[seg_idx + 1] unguarded -- out of bounds on the
            // last segment, since i holds one entry per segment.)
            let i_next = self.i[(seg_idx + 1).min(self.i.len() - 1)];
            let i_avg = 0.5 * (self.i[seg_idx] + i_next);
            let node = &self.nodes[seg_idx];
            let (i,j,k) = node.fdtd_indices;
            // Add to H-field curl (Biot-Savart approximation)
            grid.hx[(i, j, k, 1)] += i_avg * dt / grid.dy; // Simplified
            grid.hy[(i, j, k, 0)] -= i_avg * dt / grid.dx;
        }
    }

    fn solve_mna(&mut self, dt: f64, dz: f64, e_inc: &DVector<f64>) {
        let n = self.v.len();
        let g = 1.0 / self.z0;
        let c = self.z0 / (self.v_phase * self.v_phase); // C per unit length
        let l = 1.0 / (c * self.v_phase * self.v_phase); // L per unit length

        // Build MNA matrix: G * V + C * dV/dt = I_source
        // Discretized: (G + C/dt) V^{n+1} = C/dt V^n + I_source
        // For lossless line: G=0, C=1/Z0/v_phase
        let mut a = DMatrix::zeros(n, n);
        let mut b = DVector::zeros(n);

        // Interior nodes: standard FDTD equivalent
        for k in 1..n-1 {
            a[(k, k-1)] = -1.0 / (self.z0 * dz);
            a[(k, k)] = 2.0 / (self.z0 * dz) + c * dz / dt;
            a[(k, k+1)] = -1.0 / (self.z0 * dz);
            b[k] = c * dz / dt * self.v[k] + e_inc[k] * dz;
        }

        // Boundary conditions at k=0 (near end, CPU pin)
        match self.terminations.0 {
            Termination::PinModel { capacitance, inductance } => {
                // Pin model: C_pin dV/dt + V/R_pin + ... = I_wire
                a[(0, 0)] = 1.0 / (self.z0 * dz) + capacitance / dt;
                a[(0, 1)] = -1.0 / (self.z0 * dz);
                b[0] = capacitance / dt * self.v[0] + e_inc[0] * dz;
            }
            Termination::Matched(r) => {
                a[(0, 0)] = 1.0 / (self.z0 * dz) + 1.0 / r;
                a[(0, 1)] = -1.0 / (self.z0 * dz);
                b[0] = e_inc[0] * dz;
            }
            _ => {}
        }

        // Boundary at k=n-1 (far end)
        match self.terminations.1 {
            Termination::Matched(r) => {
                a[(n-1, n-2)] = -1.0 / (self.z0 * dz);
                a[(n-1, n-1)] = 1.0 / (self.z0 * dz) + 1.0 / r;
                b[n-1] = e_inc[n-1] * dz;
            }
            _ => {}
        }

        // Solve linear system
        let v_new = a.lu().solve(&b).expect("MNA solve failed");
        self.v = v_new;

        // Update currents from voltages
        for k in 0..n-1 {
            self.i[k] = (self.v[k] - self.v[k+1]) / (self.z0 * dz);
        }
    }

    /// Get differential-mode voltage at CPU pin (near end)
    pub fn pin_voltage(&self) -> f64 {
        self.v[0]
    }
}
