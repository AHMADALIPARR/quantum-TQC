// File: dodleston_anomaly.rs
// Detects unauthorized byte streams on isolated systems via reachability bounds.

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BBCMicroState {
    pub memory: [u8; 32768], // 32KB RAM
    pub pc: u16, // Program Counter
    pub acc: u8, // Accumulator
}

impl BBCMicroState {
    pub fn initial() -> Self {
        Self {
            memory: [0; 32768],
            pc: 0x0000,
            acc: 0,
        }
    }
}

pub struct DodlestonDetector {
    pub reachable: HashSet<BBCMicroState>,
    pub delta_max: f64, // Max channel-induced entropy increase
}

impl DodlestonDetector {
    pub fn new() -> Self {
        Self {
            reachable: HashSet::new(),
            delta_max: 0.5, // bits per byte (example)
        }
    }

    /// Enumerate reachable states via BFS (simplified: only memory writes)
    pub fn enumerate_reachable(&mut self, max_depth: usize) {
        let mut queue = vec![BBCMicroState::initial()];
        self.reachable.insert(BBCMicroState::initial());

        for _ in 0..max_depth {
            let mut next_queue = vec![];
            for state in &queue {
                for addr in 0..32768 {
                    for byte in 0..=255u8 {
                        let mut new_state = state.clone();
                        new_state.memory[addr] = byte;
                        if !self.reachable.contains(&new_state) {
                            self.reachable.insert(new_state.clone());
                            next_queue.push(new_state);
                        }
                    }
                }
            }
            queue = next_queue;
            if queue.is_empty() { break; }
        }
    }

    /// Compute empirical Shannon entropy of byte stream
    pub fn empirical_entropy(&self, stream: &[u8]) -> f64 {
        let mut counts = [0usize; 256];
        for &b in stream { counts[b as usize] += 1; }
        let n = stream.len() as f64;
        let mut h = 0.0;
        for &c in &counts {
            if c > 0 {
                let p = c as f64 / n;
                h -= p * p.log2();
            }
        }
        h
    }

    /// Check if stream is a Dodleston Anomaly
    pub fn is_anomaly(&self, stream: &[u8]) -> (bool, f64) {
        let h = self.empirical_entropy(stream);
        let max_reachable_entropy = (self.reachable.len() as f64).log2();
        let threshold = max_reachable_entropy + self.delta_max;
        (h > threshold, h)
    }
}

fn main() {
    let mut detector = DodlestonDetector::new();
    detector.enumerate_reachable(1);

    let stream: Vec<u8> = (0..=255).collect();
    let (anomaly, h) = detector.is_anomaly(&stream);
    println!("Empirical entropy: {:.4} bits/byte", h);
    println!("Anomaly detected: {}", anomaly);
}
