// File: fdtd_bus/src/fdtd.rs
// 3D FDTD solver with PML and thin-wire subgridding.

use nalgebra::{Vector3, Matrix3, Point3};
use ndarray::{Array3, Array4, Zip};
use crate::thin_wire::ThinWire;
use num_complex::Complex64;
use rayon::prelude::*;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub eps_r: f64,
    pub mu_r: f64,
    pub sigma: f64, // S/m
    pub sigma_m: f64, // Magnetic loss (for PML)
}

impl Material {
    pub fn vacuum() -> Self { Self { eps_r: 1.0, mu_r: 1.0, sigma: 0.0, sigma_m: 0.0 } }
    pub fn copper() -> Self { Self { eps_r: 1.0, mu_r: 1.0, sigma: 5.8e7, sigma_m: 0.0 } }
    pub fn fr4() -> Self { Self { eps_r: 4.4, mu_r: 1.0, sigma: 0.0, sigma_m: 0.0 } }
    pub fn pml(sigma_max: f64) -> Self { Self { eps_r: 1.0, mu_r: 1.0, sigma: 0.0, sigma_m: sigma_max } }
}

#[derive(Debug)]
pub struct YeeGrid {
    pub nx: usize, pub ny: usize, pub nz: usize,
    pub dx: f64, pub dy: f64, pub dz: f64,
    pub dt: f64,
    // E-field components (staggered)
    pub ex: Array4<f64>, // [nx+1, ny, nz, 3] -> Ex, Ey, Ez at (i+1/2, j, k)
    pub ey: Array4<f64>,
    pub ez: Array4<f64>,
    // H-field components
    pub hx: Array4<f64>, // [nx, ny+1, nz, 3]
    pub hy: Array4<f64>,
    pub hz: Array4<f64>,
    // Material properties at E/H nodes
    pub mat_e: Array4<Material>, // [nx, ny, nz]
    pub mat_h: Array4<Material>,
    // PML coefficients
    pub ca_ex: Array3<f64>, pub cb_ex: Array3<f64>,
    // ... (similar for all 6 components)
}

impl YeeGrid {
    pub fn new(nx: usize, ny: usize, nz: usize, dx: f64, dy: f64, dz: f64, pml_thickness: usize) -> Self {
        let dt = Self::courant_dt(dx, dy, dz) * 0.99; // Safety factor
        let ex = Array4::zeros((nx+1, ny, nz, 3));
        let ey = Array4::zeros((nx, ny+1, nz, 3));
        let ez = Array4::zeros((nx, ny, nz+1, 3));
        let hx = Array4::zeros((nx, ny+1, nz, 3));
        let hy = Array4::zeros((nx+1, ny, nz, 3));
        let hz = Array4::zeros((nx+1, ny+1, nz, 3));
        let mat_e = Array4::from_elem((nx, ny, nz, 1), Material::vacuum());
        let mat_h = mat_e.clone();

        // PML coefficients (simplified uniaxial PML)
        let mut ca_ex = Array3::ones((nx, ny, nz));
        let mut cb_ex = Array3::zeros((nx, ny, nz));
        // ... initialize PML grading in boundary regions

        Self { nx, ny, nz, dx, dy, dz, dt, ex, ey, ez, hx, hy, hz, mat_e, mat_h, ca_ex, cb_ex }
    }

    fn courant_dt(dx: f64, dy: f64, dz: f64) -> f64 {
        1.0 / (299792458.0 * (1.0/dx*dx + 1.0/dy*dy + 1.0/dz*dz).sqrt())
    }

    /// Update H-field (curl E)
    pub fn update_h(&mut self) {
        // Standard Yee curl equations with material coefficients
        // Parallelized over z-slices
        // ... curl E update for Hx
        // Similar for Hy, Hz
    }

    /// Update E-field (curl H) with conductivity
    pub fn update_e(&mut self, sources: &[Box<dyn Source>]) {
        // Apply sources first (hard source or TF/SF)
        for src in sources {
            src.inject_e(self);
        }
        // Curl H update
        // ...
    }

    /// Thin-wire update for bus traces (subgridding)
    pub fn update_thin_wires(&mut self, wires: &mut [ThinWire]) {
        for wire in wires {
            wire.update_coupled(self);
        }
    }

    /// Step one time step
    pub fn step(&mut self, sources: &[Box<dyn Source>], wires: &mut [ThinWire]) {
        self.update_h();
        self.update_e(sources);
        self.update_thin_wires(wires);
    }
}

/// Source trait
pub trait Source: Send + Sync {
    fn inject_e(&self, grid: &mut YeeGrid);
    fn inject_h(&self, grid: &mut YeeGrid);
}

/// Plane wave TF/SF source
pub struct PlaneWaveSource {
    pub direction: Vector3<f64>,
    pub polarization: Vector3<f64>,
    pub frequency: f64,
    pub amplitude: f64,
    pub tf_sf_box: (Point3<usize>, Point3<usize>), // TF/SF boundary indices
}

impl Source for PlaneWaveSource {
    fn inject_e(&self, grid: &mut YeeGrid) {
        // TF/SF correction on E-field at boundary
    }
    fn inject_h(&self, grid: &mut YeeGrid) {
        // TF/SF correction on H-field
    }
}
