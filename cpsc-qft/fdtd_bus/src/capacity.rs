// File: fdtd_bus/src/capacity.rs
// Covert channel capacity and minimal energy reconstruction.

use ndarray::{Array1, Array2};
use num_complex::Complex64;
use rustfft::{FftPlanner, num_complex::Complex};
use crate::fdtd::YeeGrid;
use crate::thin_wire::{Termination, ThinWire};
use nalgebra::{DMatrix, DVector};

/// Frequency response measurement
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrequencyResponse {
    pub freq: Array1<f64>, // Hz
    pub h: Array1<Complex64>, // V_out / E_inc
    pub noise_floor: Array1<f64>, // V^2/Hz (thermal + quantization)
}

impl FrequencyResponse {
    /// Compute from time-domain simulation
    pub fn from_td(td_v: &[f64], td_e_inc: &[f64], dt: f64) -> Self {
        let n = td_v.len();
        let mut planner = FftPlanner::<f64>::new();
        let fft = planner.plan_fft_forward(n);

        let mut v_c: Vec<Complex<f64>> = td_v.iter().map(|&x| Complex::new(x, 0.0)).collect();
        let mut e_c: Vec<Complex<f64>> = td_e_inc.iter().map(|x| Complex::new(*x, 0.0)).collect();
        fft.process(&mut v_c);
        fft.process(&mut e_c);

        let freq: Array1<f64> = Array1::from_shape_fn(n, |k| k as f64 / (n as f64 * dt));
        let h = Array1::from_shape_fn(n, |k| {
            if e_c[k].norm() > 1e-12 {
                Complex64::new(v_c[k].re / e_c[k].re, v_c[k].im / e_c[k].im) // Simplified
            } else { Complex64::new(0.0, 0.0) }
        });

        // Noise floor: thermal (kTB) + 6502 input noise (~100μV/√Hz)
        let noise_floor = Array1::from_shape_fn(n, |k| {
            let f = freq[k];
            let thermal = 1.38e-23 * 300.0 * 50.0; // 50Ω equivalent
            let quant = (0.5f64 / 256.0).powi(2) / (2.0e6); // 8-bit, 2MHz clock
            thermal + quant
        });

        Self { freq, h, noise_floor }
    }

    /// Shannon capacity (bits/sec)
    pub fn capacity(&self, p_tx: f64) -> f64 {
        // Integrate over positive frequencies only
        let mut cap = 0.0;
        for k in 1..self.freq.len()/2 {
            let df = self.freq[1] - self.freq[0];
            let snr = self.h[k].norm_sqr() * p_tx / self.noise_floor[k];
            if snr > 0.0 {
                cap += df * (1.0 + snr).log2();
            }
        }
        cap
    }

    /// Minimal transmit energy to achieve target voltage waveform at pin
    pub fn minimal_energy(&self, target_v: &[f64], dt: f64) -> f64 {
        // Inverse problem: min ||E_inc||^2 s.t. H * E_inc = V_target
        // Frequency domain: E_inc(f) = V_target(f) / H(f)
        // Regularized: E_inc(f) = V_target(f) * H*(f) / (|H(f)|^2 + λ)
        let n = target_v.len();
        let mut planner = FftPlanner::<f64>::new();
        let fft = planner.plan_fft_forward(n);
        let ifft = planner.plan_fft_inverse(n);

        let mut v_c: Vec<Complex<f64>> = target_v.iter().map(|&x| Complex::new(x, 0.0)).collect();
        fft.process(&mut v_c);

        let mut e_c = vec![Complex::new(0.0, 0.0); n];
        let lambda = 1e-6; // Tikhonov regularization
        for k in 0..n {
            let h = self.h[k];
            let denom = h.norm_sqr() + lambda;
            if denom > 0.0 {
                e_c[k] = v_c[k] * h.conj() / Complex::new(denom, 0.0);
            }
        }
        ifft.process(&mut e_c);

        // Energy = ∫ |E_inc(t)|^2 dt
        let energy: f64 = e_c.iter().map(|c| c.norm_sqr() * dt).sum();
        energy
    }
}

/// Adjoint FDTD for gradient-based optimization
pub struct AdjointFDTD {
    pub forward_grid: YeeGrid,
    pub adjoint_grid: YeeGrid,
    pub wires: Vec<ThinWire>,
}

impl AdjointFDTD {
    pub fn new(geometry: &crate::geometry::BbcMicroGeometry, dx: f64) -> Self {
        // Same construction as DodlestonEMModel::new: one Yee grid per
        // pass over the geometry's bounding box, thin wires for every
        // trace, mapped onto the forward grid.
        let (min, max) = geometry.bounding_box;
        let nx = ((max.x - min.x) / dx).ceil() as usize;
        let ny = ((max.y - min.y) / dx).ceil() as usize;
        let nz = ((max.z - min.z) / dx).ceil() as usize;
        let forward_grid = YeeGrid::new(nx, ny, nz, dx, dx, dx, 10);
        let adjoint_grid = YeeGrid::new(nx, ny, nz, dx, dx, dx, 10);

        let mut wires = Vec::new();
        for trace in &geometry.traces {
            let mut wire = ThinWire::new_from_trace(trace, &geometry.stackup, dx);
            wire.map_to_fdtd(&forward_grid);
            wires.push(wire);
        }

        Self { forward_grid, adjoint_grid, wires }
    }

    /// Pin wire for the adjoint experiment: the first wire, the same
    /// one DodlestonEMModel::characterize reads as the A0 pin.
    fn pin_wire(&mut self) -> &mut ThinWire {
        &mut self.wires[0]
    }

    /// Forward experiment: drive the pin wire with the incident-field
    /// samples `drive` (E_z at the pin node, one per step) through its
    /// own coupled update and record the pin voltage after each step.
    /// The wire state is reset first, so runs are repeatable.
    fn forward_pin_response(&mut self, drive: &[f64]) -> Vec<f64> {
        let (pi, pj, pk) = self.wires[0].nodes[0].fdtd_indices;
        {
            let wire = self.pin_wire();
            wire.v = DVector::zeros(wire.v.len());
            wire.i = DVector::zeros(wire.i.len());
        }
        let mut u = Vec::with_capacity(drive.len());
        for &e in drive {
            self.forward_grid.ez[(pi, pj, pk, 2)] = e;
            let grid = &mut self.forward_grid;
            self.wires[0].update_coupled(grid);
            u.push(self.wires[0].pin_voltage());
        }
        u
    }

    /// The MNA step matrices of the pin wire, mirroring
    /// ThinWire::solve_mna coefficient for coefficient: one step is
    /// v' = A^-1 (D v + dz * e_inc), with A tridiagonal, D diagonal.
    /// Returns (A, diag(D), segment spacing dz).
    fn pin_mna_matrices(&self) -> (DMatrix<f64>, Vec<f64>, f64) {
        let wire = &self.wires[0];
        let n = wire.v.len();
        assert!(n >= 2, "pin wire must have at least two nodes");
        let dt = self.forward_grid.dt;
        let dz = (wire.nodes[1].position - wire.nodes[0].position).norm();
        let g = 1.0 / (wire.z0 * dz);
        let c = wire.z0 / (wire.v_phase * wire.v_phase); // C per unit length, as in solve_mna

        let mut a = DMatrix::zeros(n, n);
        let mut d = vec![0.0; n];
        for k in 1..n - 1 {
            a[(k, k - 1)] = -g;
            a[(k, k)] = 2.0 * g + c * dz / dt;
            a[(k, k + 1)] = -g;
            d[k] = c * dz / dt;
        }
        match wire.terminations.0 {
            Termination::PinModel { capacitance, .. } => {
                a[(0, 0)] = g + capacitance / dt;
                a[(0, 1)] = -g;
                d[0] = capacitance / dt;
            }
            Termination::Matched(r) => {
                a[(0, 0)] = g + 1.0 / r;
                a[(0, 1)] = -g;
            }
            _ => {} // solve_mna leaves this row empty as well
        }
        match wire.terminations.1 {
            Termination::Matched(r) => {
                a[(n - 1, n - 2)] = -g;
                a[(n - 1, n - 1)] = g + 1.0 / r;
            }
            _ => {} // solve_mna handles only Matched at the far end
        }
        (a, d, dz)
    }

    /// Compute gradient of ||V_pin - V_target||^2 w.r.t. source amplitude
    ///
    /// Experiment: the pin wire is driven at its pin node by incident
    /// samples e_t (E_z, V/m); V_pin(t) is the pin voltage after step
    /// t and J = sum_t (V_pin(t) - target_t)^2. This returns the
    /// per-sample gradient dJ/de_t, computed by the discrete adjoint
    /// of the wire's MNA recursion:
    ///   1. Forward simulation, storing V_pin at each step.
    ///   2. Adjoint recursion backward: lambda_t = M^T lambda_{t+1}
    ///      + 2 (V_pin(t) - target_t) e_0, where M = A^-1 D is the
    ///      MNA step matrix.
    ///   3. Gradient sample: lambda_{t+1} . (A^-1 dz e_0) -- the
    ///      forward/adjoint field correlation at the pin, per step.
    /// For a constant-amplitude source (e_t = A for all t), dJ/dA is
    /// the sum of the returned samples.
    pub fn gradient(&mut self, target: &[f64]) -> Vec<f64> {
        let steps = target.len();
        if steps == 0 {
            return Vec::new();
        }
        if self.wires.is_empty() {
            // No pin wire: V_pin is identically zero, J is constant.
            return vec![0.0; steps];
        }

        // 1. Forward simulation with the unit source (e_t = 1).
        let u = self.forward_pin_response(&vec![1.0; steps]);

        // MNA step matrices and the drive vector A^-1 (dz e_0).
        let (a, d, dz) = self.pin_mna_matrices();
        let lu = a.clone().lu();
        let lu_t = a.transpose().lu();
        let mut e0 = DVector::zeros(self.wires[0].v.len());
        e0[0] = dz;
        let g_vec = lu.solve(&e0).expect("adjoint MNA matrix is singular");

        // 2./3. Backward adjoint recursion and correlation.
        let n = self.wires[0].v.len();
        let mut lambda = DVector::zeros(n);
        let mut grad = vec![0.0; steps];
        for t in (0..steps).rev() {
            let residual = u[t] - target[t];
            let mu = lu_t
                .solve(&lambda)
                .expect("adjoint MNA matrix is singular");
            let mut next = DVector::zeros(n);
            for k in 0..n {
                next[k] = d[k] * mu[k];
            }
            next[0] += 2.0 * residual;
            lambda = next;
            grad[t] = lambda.dot(&g_vec);
        }
        grad
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{BbcMicroGeometry, Layer, Stackup, Trace};
    use nalgebra::Point3;

    /// One short z-aligned trace in a small box: 5 wire nodes at
    /// 1 mm spacing, pin at the origin end.
    fn test_model() -> AdjointFDTD {
        let geom = BbcMicroGeometry {
            stackup: Stackup::default(),
            pins: Vec::new(),
            traces: vec![Trace {
                name: "T".to_string(),
                start: Point3::new(0.0, 0.0, 0.0),
                end: Point3::new(0.0, 0.0, 4e-3),
                width: 0.25e-3,
                layer: Layer::Top,
            }],
            ground_plane_z: 0.0,
            power_plane_z: 0.0,
            bounding_box: (
                Point3::new(-2e-3, -2e-3, -1e-3),
                Point3::new(2e-3, 2e-3, 5e-3),
            ),
        };
        AdjointFDTD::new(&geom, 1e-3)
    }

    fn objective(model: &mut AdjointFDTD, drive: &[f64], target: &[f64]) -> f64 {
        let u = model.forward_pin_response(drive);
        u.iter()
            .zip(target.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum()
    }

    #[test]
    fn adjoint_gradient_matches_finite_differences() {
        let steps = 40;
        let target: Vec<f64> = (0..steps).map(|t| 0.5 * (t as f64 * 0.3).sin()).collect();
        let mut model = test_model();
        let grad = model.gradient(&target);
        assert_eq!(grad.len(), steps);

        // Scalar amplitude (constant source): dJ/dA = sum of samples.
        let h = 1e-6;
        let jp = objective(&mut model, &vec![1.0 + h; steps], &target);
        let jm = objective(&mut model, &vec![1.0 - h; steps], &target);
        let fd = (jp - jm) / (2.0 * h);
        let adj: f64 = grad.iter().sum();
        assert!(
            (fd - adj).abs() <= 1e-4 * fd.abs().max(1.0),
            "adjoint {adj} vs finite-difference {fd}"
        );

        // Per-sample check at one interior step pins the indexing.
        let k = 17;
        let mut dp = vec![1.0; steps];
        let mut dm = vec![1.0; steps];
        dp[k] += h;
        dm[k] -= h;
        let fd_k = (objective(&mut model, &dp, &target)
            - objective(&mut model, &dm, &target))
            / (2.0 * h);
        assert!(
            (fd_k - grad[k]).abs() <= 1e-4 * fd_k.abs().max(1.0),
            "sample {k}: adjoint {} vs finite-difference {fd_k}",
            grad[k]
        );
    }

    #[test]
    fn gradient_is_zero_when_response_matches_target() {
        let steps = 24;
        let mut model = test_model();
        let target = model.forward_pin_response(&vec![1.0; steps]);
        let grad = model.gradient(&target);
        assert!(grad.iter().all(|g| g.abs() < 1e-12), "grad {grad:?}");
    }
}
