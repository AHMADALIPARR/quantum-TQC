// File: fdtd_bus/src/dodleston_integration.rs
// Connects EM channel model to Dodleston anomaly detection.

use crate::geometry::BbcMicroGeometry;
use crate::fdtd::YeeGrid;
use crate::thin_wire::ThinWire;
use crate::capacity::{FrequencyResponse, AdjointFDTD};
use crate::dodleston::DodlestonDetector; // From previous artifact
use ndarray::Array1;
use std::f64::consts::PI;

pub struct DodlestonEMModel {
    pub geometry: BbcMicroGeometry,
    pub grid: YeeGrid,
    pub wires: Vec<ThinWire>,
    pub freq_response: Option<FrequencyResponse>,
    pub capacity_bps: f64,
    pub delta_max_bits_per_byte: f64,
}

impl DodlestonEMModel {
    pub fn new(dx: f64) -> Self {
        let geometry = BbcMicroGeometry::canonical();
        let (min, max) = geometry.bounding_box;
        let nx = ((max.x - min.x) / dx).ceil() as usize;
        let ny = ((max.y - min.y) / dx).ceil() as usize;
        let nz = ((max.z - min.z) / dx).ceil() as usize;
        let mut grid = YeeGrid::new(nx, ny, nz, dx, dx, dx, 10); // 10-cell PML

        // Map materials
        Self::assign_materials(&mut grid, &geometry);

        // Create thin wires for all traces
        let mut wires = Vec::new();
        for trace in &geometry.traces {
            let mut wire = ThinWire::new_from_trace(trace, &geometry.stackup, dx);
            wire.map_to_fdtd(&grid);
            wires.push(wire);
        }

        Self {
            geometry, grid, wires,
            freq_response: None,
            capacity_bps: 0.0,
            delta_max_bits_per_byte: 0.0,
        }
    }

    fn assign_materials(grid: &mut YeeGrid, geom: &BbcMicroGeometry) {
        // Fill mat_e based on stackup layers
        // ...
    }

    /// Run characterization simulation (plane wave sweep)
    pub fn characterize(&mut self, freq_range: (f64, f64), n_freq: usize) {
        let (f_min, f_max) = freq_range;
        let mut all_v = Vec::new();
        let mut all_e = Vec::new();

        for f_idx in 0..n_freq {
            let f = f_min + (f_max - f_min) * f_idx as f64 / (n_freq - 1) as f64;
            let source = crate::fdtd::PlaneWaveSource {
                frequency: f,
                amplitude: 1.0, // 1 V/m
                direction: nalgebra::Vector3::new(0.0, 0.0, -1.0), // Top illumination
                polarization: nalgebra::Vector3::new(1.0, 0.0, 0.0),
                tf_sf_box: (nalgebra::Point3::new(10,10,10), nalgebra::Point3::new(self.grid.nx-10, self.grid.ny-10, self.grid.nz-10)),
            };

            let amplitude = source.amplitude;
            let sources: Vec<Box<dyn crate::fdtd::Source>> = vec![Box::new(source)];

            // Run FDTD for 10 cycles at this frequency
            let n_steps = (10.0 / f / self.grid.dt).ceil() as usize;
            let mut v_pin = Vec::new();
            let mut e_inc = Vec::new();

            for step in 0..n_steps {
                self.grid.step(&sources, &mut self.wires);
                if step > n_steps / 2 { // Steady state
                    v_pin.push(self.wires[0].pin_voltage()); // A0 pin
                    e_inc.push(amplitude * (2.0 * PI * f * step as f64 * self.grid.dt).sin());
                }
            }
            all_v.extend(v_pin);
            all_e.extend(e_inc);
        }

        self.freq_response = Some(FrequencyResponse::from_td(&all_v, &all_e, self.grid.dt));
        if let Some(fr) = &self.freq_response {
            self.capacity_bps = fr.capacity(1e-3); // 1mW incident power
            // Delta max: capacity * observation_time / bytes_observed
            self.delta_max_bits_per_byte = self.capacity_bps * 1e-3 / 8.0; // Rough estimate
        }
    }

    /// Check if a byte stream is explainable by EM coupling
    pub fn explain_stream(&mut self, stream: &[u8], detector: &mut DodlestonDetector) -> (bool, f64) {
        detector.delta_max = self.delta_max_bits_per_byte;
        detector.enumerate_reachable(2); // Update reachable set
        detector.is_anomaly(stream)
    }

    /// Compute minimal external field energy to produce target stream
    pub fn minimal_field_energy(&mut self, target_stream: &[u8]) -> f64 {
        // Convert byte stream to target pin voltage waveform
        // (Simplified: assume each byte drives address bus pattern)
        let target_v = self.stream_to_voltage(target_stream);
        if let Some(fr) = &self.freq_response {
            fr.minimal_energy(&target_v, self.grid.dt)
        } else {
            f64::INFINITY
        }
    }

    fn stream_to_voltage(&self, stream: &[u8]) -> Vec<f64> {
        // Map bytes to address bus transitions (simplified)
        let mut v = Vec::new();
        for &byte in stream {
            for bit in 0..8 {
                v.push(if (byte >> bit) & 1 == 1 { 5.0 } else { 0.0 }); // 5V logic
            }
        }
        v
    }
}
