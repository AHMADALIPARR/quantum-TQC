// File: fdtd_bus/src/main.rs
// End-to-end Dodleston EM analysis.

use fdtd_bus::{DodlestonEMModel, DodlestonDetector};

fn main() {
    println!("=== Dodleston EM Side-Channel Analysis ===");

    // 1. Build model (cell size 1mm -> ~50k cells, manageable)
    let mut model = DodlestonEMModel::new(1.0e-3); // 1mm cells
    println!("Grid: {}x{}x{}", model.grid.nx, model.grid.ny, model.grid.nz);
    println!("Wires: {}", model.wires.len());

    // 2. Characterize channel (1MHz to 500MHz)
    println!("Running FDTD characterization sweep...");
    model.characterize((1e6, 500e6), 50);
    println!("Covert channel capacity: {:.2} bps", model.capacity_bps);
    println!("Delta_max (bits/byte): {:.6}", model.delta_max_bits_per_byte);

    // 3. Test anomaly detection
    let mut detector = DodlestonDetector::new();
    let suspicious_stream: Vec<u8> = (0..=255).collect(); // Uniform entropy
    let (anomaly, entropy) = model.explain_stream(&suspicious_stream, &mut detector);
    println!("Stream entropy: {:.4} bits/byte", entropy);
    println!("Anomaly (unexplainable by EM): {}", anomaly);

    // 4. Minimal energy for a reachable pattern
    let reachable_stream = vec![0xAA; 32]; // Alternating pattern
    let energy = model.minimal_field_energy(&reachable_stream);
    println!("Minimal EM energy for 0xAA pattern: {:.2e} J", energy);

    // 5. Export frequency response for analysis
    if let Some(fr) = &model.freq_response {
        std::fs::write("freq_response.json", serde_json::to_string_pretty(fr).unwrap()).unwrap();
        println!("Exported freq_response.json");
    }
}
