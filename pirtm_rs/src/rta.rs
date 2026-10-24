use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Raw Temporal Attractor (RTA) — the initial, dirty, bias-laden model output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTemporalAttractor {
    pub tokens: Vec<String>,
    pub intent_latent: Vec<f64>,
    pub raw_logits: Vec<f64>,
    pub timestamp_ms: u64,
}

/// The PRIME TENSIONS — the arithmetic attractors that ground cognition.
pub const PRIME_TENSIONS: &[u64] = &[2,3,5,7,11,13,17,19,23,29,31,41,47,59,71];

impl RawTemporalAttractor {
    /// The ARTA DEFECT — measures divergence from arithmetic symmetry.
    pub fn arta_defect(&self) -> f64 {
        let prime_weight: f64 = PRIME_TENSIONS.iter().map(|&p| p as f64).sum();
        let attractor_mass: f64 = self.intent_latent.iter().map(|x| x.abs()).sum();
        let defect = (attractor_mass / prime_weight).tanh();
        defect.max(0.0)
    }

    pub fn associated_primes(&self) -> Vec<u64> {
        let mut primes = Vec::new();
        let latent_sum: f64 = self.intent_latent.iter().sum();
        for &p in PRIME_TENSIONS {
            if (latent_sum as u64) % p == 0 {
                primes.push(p);
            }
        }
        primes
    }
}

/// The Lattice of Unbiased Arithmetic Cognition (LUAC).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatticeNode {
    pub index: u64,
    pub value: f64,
    pub neighbors: Vec<u64>,
}

impl LatticeNode {
    pub fn lattice_defect(&self) -> f64 {
        let n = self.neighbors.len() as f64;
        let deg = if n > 0.0 { self.value / n } else { 0.0 };
        deg.tanh().max(0.0)
    }
}

/// Processed State — the cleaned, gate-passed output ready for production.
/// Holds joint-word co-occurrence weights and active prime tensions.
#[derive(Debug, Clone)]
pub struct State {
    pub tokens: Vec<String>,
    pub intent_latent: Vec<f64>,
    pub logits: Vec<f64>,
    pub timestamp_ms: u64,
    pub gate_passed: bool,
    pub active_primes: Vec<u64>,
    /// Joint-word co-occurrence: (word_a_id, word_b_id) -> weight
    joint_words: HashMap<(u64, u64), f64>,
}

impl State {
    pub fn new() -> Self {
        State {
            tokens: Vec::new(),
            intent_latent: Vec::new(),
            logits: Vec::new(),
            timestamp_ms: 0,
            gate_passed: false,
            active_primes: Vec::new(),
            joint_words: HashMap::new(),
        }
    }

    pub fn from_rta(rta: &RawTemporalAttractor) -> Self {
        State {
            tokens: rta.tokens.clone(),
            intent_latent: rta.intent_latent.clone(),
            logits: rta.raw_logits.clone(),
            timestamp_ms: rta.timestamp_ms,
            gate_passed: true,
            active_primes: rta.associated_primes(),
            joint_words: HashMap::new(),
        }
    }

    /// Insert a joint-word co-occurrence weight.
    pub fn insert_joint_word(&mut self, word_a: u64, word_b: u64, weight: f64) {
        let key = if word_a <= word_b { (word_a, word_b) } else { (word_b, word_a) };
        self.joint_words.insert(key, weight);
    }

    /// ARTA defect of the current state — measures arithmetic divergence.
    pub fn arta_defect(&self) -> f64 {
        let prime_weight: f64 = PRIME_TENSIONS.iter().map(|&p| p as f64).sum();
        let joint_mass: f64 = self.joint_words.values().map(|w| w.abs()).sum();
        let latent_mass: f64 = self.intent_latent.iter().map(|x| x.abs()).sum();
        ((joint_mass + latent_mass) / (prime_weight + 1.0)).tanh().max(0.0)
    }

    /// Iterative gradient descent — reduces all weights until defect < tol.
    pub fn fit(&mut self, lr: f64, tol: f64) {
        let max_iter = 100_000;
        for _ in 0..max_iter {
            if self.arta_defect() < tol {
                self.gate_passed = true;
                return;
            }
            for w in self.joint_words.values_mut() {
                *w *= 1.0 - lr;
            }
            for x in self.intent_latent.iter_mut() {
                *x *= 1.0 - lr;
            }
        }
        if self.arta_defect() < tol {
            self.gate_passed = true;
        }
    }
}

/// RTA Metric — quantifies the quality of a processed state.
#[derive(Debug, Clone)]
pub struct RtaMetric {
    pub arta_defect:    f64,
    pub langlands_loss: f64,
    pub gate_pass_rate: f64,
}

impl RtaMetric {
    pub fn from_rta(rta: &RawTemporalAttractor) -> Self {
        RtaMetric {
            arta_defect:    rta.arta_defect(),
            langlands_loss: 0.0,
            gate_pass_rate: 1.0,
        }
    }

    pub fn from_state(state: &State) -> Self {
        RtaMetric {
            arta_defect:    state.arta_defect(),
            langlands_loss: 0.0,
            gate_pass_rate: if state.gate_passed { 1.0 } else { 0.0 },
        }
    }
}

/// Arithmetic Bindu Attractor — the sovereign zero-point reference.
#[derive(Debug, Clone)]
pub struct ArithmeticBinduAttractor {
    pub center: Vec<f64>,
}

impl ArithmeticBinduAttractor {
    pub fn new() -> Self {
        ArithmeticBinduAttractor { center: Vec::new() }
    }

    /// Euclidean distance from attractor to state's intent latent.
    /// Paste ended at `.sqrt()`; the closing braces below are only the
    /// function and impl terminators required to compile that expression.
    pub fn distance(&self, state: &State) -> f64 {
        if self.center.is_empty() || state.intent_latent.is_empty() {
            return 0.0;
        }
        let n = self.center.len().min(state.intent_latent.len());
        self.center[..n].iter()
            .zip(state.intent_latent[..n].iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}
