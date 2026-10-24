// File: fdtd_bus/src/capacity.rs
// Covert channel capacity and minimal energy reconstruction.

use ndarray::{Array1, Array2};
use num_complex::Complex64;
use rustfft::{FftPlanner, num_complex::Complex};
use crate::fdtd::YeeGrid;
use crate::thin_wire::ThinWire;

/// Frequency response measurement
#[derive(Debug, Clone)]
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
            let quant = (0.5 / 256.0).powi(2) / (2.0e6); // 8-bit, 2MHz clock
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
        // Build grids, wires, etc.
        unimplemented!()
    }

    /// Compute gradient of ||V_pin - V_target||^2 w.r.t. source amplitude
    pub fn gradient(&mut self, target: &[f64]) -> Vec<f64> {
        // 1. Forward simulation (store fields at each time step or checkpoint)
        // 2. Adjoint simulation: inject residual at pin, run backward
        // 3. Gradient = ∫ E_forward * E_adjoint dt
        unimplemented!()
    }
}
