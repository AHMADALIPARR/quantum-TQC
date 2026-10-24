// File: fdtd_bus/src/geometry.rs
// Physical geometry of BBC Micro 6502 bus region.

use nalgebra::{Vector3, Point3};
use serde::{Serialize, Deserialize};

/// 6502 Pin definitions (40-pin DIP)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pin6502 {
    Vss, // 1, 21
    RdY, // 2
    Rdy, // 3
    Irq, // 4
    Nmi, // 6
    Sync, // 7
    Vcc, // 8, 26
    A0, A1, A2, A3, A4, A5, A6, A7, // 9-16
    A8, A9, A10, A11, A12, A13, A14, A15, // 17-25 (skipping Vss/Vcc)
    D0, D1, D2, D3, D4, D5, D6, D7, // 26-33 (skipping Vcc)
    Rw, // 34
    Nc, // 35
    Phi0, // 37
    Phi1, // 38
    Res, // 40
    Phi2, // 39
    So, // 36
}

impl Pin6502 {
    pub fn pin_number(&self) -> u8 {
        use Pin6502::*;
        match self {
            Vss => 1, RdY => 2, Rdy => 3, Irq => 4, Nmi => 6, Sync => 7, Vcc => 8,
            A0 => 9, A1 => 10, A2 => 11, A3 => 12, A4 => 13, A5 => 14, A6 => 15, A7 => 16,
            A8 => 17, A9 => 18, A10 => 19, A11 => 20, A12 => 22, A13 => 23, A14 => 24, A15 => 25,
            D0 => 26, D1 => 27, D2 => 28, D3 => 29, D4 => 30, D5 => 31, D6 => 32, D7 => 33,
            Rw => 34, So => 36, Nc => 35, Phi2 => 39, Phi1 => 38, Phi0 => 37, Res => 40,
        }
    }

    pub fn is_address(&self) -> bool {
        matches!(self, Self::A0 | Self::A1 | Self::A2 | Self::A3 | Self::A4 | Self::A5 | Self::A6 | Self::A7 |
                       Self::A8 | Self::A9 | Self::A10 | Self::A11 | Self::A12 | Self::A13 | Self::A14 | Self::A15)
    }
    pub fn is_data(&self) -> bool {
        matches!(self, Self::D0 | Self::D1 | Self::D2 | Self::D3 | Self::D4 | Self::D5 | Self::D6 | Self::D7)
    }
    pub fn is_control(&self) -> bool {
        matches!(self, Self::Rw | Self::Phi0 | Self::Phi1 | Self::Phi2 | Self::Sync | Self::Rdy | Self::Irq | Self::Nmi | Self::Res | Self::So)
    }
}

/// PCB Stackup (BBC Micro Issue 4/7 typical)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stackup {
    pub copper_thickness: f64, // 35 μm
    pub fr4_thickness: f64, // 1.6 mm
    pub fr4_er: f64, // 4.4
    pub fr4_tand: f64, // 0.02
    pub solder_mask_thickness: f64,// 25 μm
    pub solder_mask_er: f64, // 3.5
}

impl Default for Stackup {
    fn default() -> Self {
        Self {
            copper_thickness: 35e-6,
            fr4_thickness: 1.6e-3,
            fr4_er: 4.4,
            fr4_tand: 0.02,
            solder_mask_thickness: 25e-6,
            solder_mask_er: 3.5,
        }
    }
}

/// Trace geometry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trace {
    pub name: String,
    pub start: Point3<f64>, // Pin exit point
    pub end: Point3<f64>, // Via / connector
    pub width: f64, // 0.25 mm typical
    pub layer: Layer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Layer { Top, Bottom, Inner1, Inner2 }

/// Complete geometry for FDTD meshing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbcMicroGeometry {
    pub stackup: Stackup,
    pub pins: Vec<(Pin6502, Point3<f64>)>, // Pin center positions
    pub traces: Vec<Trace>,
    pub ground_plane_z: f64, // Inner1 typically
    pub power_plane_z: f64, // Inner2 typically
    pub bounding_box: (Point3<f64>, Point3<f64>),
}

impl BbcMicroGeometry {
    /// Create canonical BBC Micro 6502 region geometry
    pub fn canonical() -> Self {
        let mut pins = Vec::new();
        // 40-pin DIP: 52.3mm x 15.2mm body, pins on 2.54mm pitch
        let pin_pitch = 2.54e-3;
        let row_spacing = 15.24e-3;
        let start_x = -26.15e-3;
        let start_y = -7.62e-3;

        for pin_num in 1..=40 {
            let (row, idx) = if pin_num <= 20 { (0, pin_num - 1) } else { (1, 40 - pin_num) };
            let x = start_x + idx as f64 * pin_pitch;
            let y = start_y + row as f64 * row_spacing;
            let pin = Self::pin_from_number(pin_num);
            pins.push((pin, Point3::new(x, y, 0.0)));
        }

        // Traces: simplified straight lines to edge of region
        let mut traces = Vec::new();
        for (pin, pos) in &pins {
            if pin.is_address() || pin.is_data() || pin.is_control() {
                let end = Point3::new(pos.x, pos.y + 30e-3, 0.0); // 30mm trace length
                traces.push(Trace {
                    name: format!("{:?}", pin),
                    start: *pos,
                    end,
                    width: 0.25e-3,
                    layer: Layer::Top,
                });
            }
        }

        Self {
            stackup: Stackup::default(),
            pins,
            traces,
            ground_plane_z: -0.8e-3, // Mid-FR4
            power_plane_z: -0.4e-3,
            bounding_box: (
                Point3::new(-30e-3, -15e-3, -2e-3),
                Point3::new(30e-3, 30e-3, 2e-3),
            ),
        }
    }

    fn pin_from_number(n: u8) -> Pin6502 {
        use Pin6502::*;
        match n {
            1 => Vss, 2 => RdY, 3 => Rdy, 4 => Irq, 6 => Nmi, 7 => Sync, 8 => Vcc,
            9 => A0, 10 => A1, 11 => A2, 12 => A3, 13 => A4, 14 => A5, 15 => A6, 16 => A7,
            17 => A8, 18 => A9, 19 => A10, 20 => A11, 22 => A12, 23 => A13, 24 => A14, 25 => A15,
            26 => D0, 27 => D1, 28 => D2, 29 => D3, 30 => D4, 31 => D5, 32 => D6, 33 => D7,
            34 => Rw, 35 => Nc, 36 => So, 37 => Phi0, 38 => Phi1, 39 => Phi2, 40 => Res,
            _ => Vss,
        }
    }
}
