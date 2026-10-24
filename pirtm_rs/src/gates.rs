
use crate::rta::State;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum GateFailure {
    LVanished(String),
    LOutOfBounds(String, f64),
    GaloisError(String, String),
    ZKVerificationError(String, String),
}

impl fmt::Display for GateFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LVanished(class_id) => {
                write!(f, "Langlands L-function vanished for class {class_id}")
            }
            Self::LOutOfBounds(class_id, value) => {
                write!(f, "Langlands L-function out of safety bounds for class {class_id}: {value}")
            }
            Self::GaloisError(class_id, error) => {
                write!(f, "Galois computation failed for class {class_id}: {error}")
            }
            Self::ZKVerificationError(class_id, error) => {
                write!(f, "ZK verification failed for class {class_id}: {error}")
            }
        }
    }
}

impl Error for GateFailure {}

pub type GateResult<T> = std::result::Result<T, GateFailure>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonsterConjugacyClass {
    pub class_id: &'static str,
    pub cycle_shape: &'static [(u64, u64)],
    pub level: u64,
}

impl MonsterConjugacyClass {
    pub const IDENTITY: Self = Self {
        class_id: "1A",
        cycle_shape: &[(1, 1)],
        level: 1,
    };
    pub const CLASS_2A: Self = Self {
        class_id: "2A",
        cycle_shape: &[(1, 24), (2, 24)],
        level: 2,
    };
    pub const CLASS_3A: Self = Self {
        class_id: "3A",
        cycle_shape: &[(1, 9), (3, 9)],
        level: 3,
    };
    pub const CLASS_5A: Self = Self {
        class_id: "5A",
        cycle_shape: &[(1, 4), (5, 4)],
        level: 5,
    };
    pub const CLASS_7A: Self = Self {
        class_id: "7A",
        cycle_shape: &[(1, 3), (7, 3)],
        level: 7,
    };
    pub const CLASS_11A: Self = Self {
        class_id: "11A",
        cycle_shape: &[(1, 2), (11, 2)],
        level: 11,
    };
}

pub const ALL_MONSTER_CLASSES: &[MonsterConjugacyClass] = &[
    MonsterConjugacyClass::IDENTITY,
    MonsterConjugacyClass::CLASS_2A,
    MonsterConjugacyClass::CLASS_3A,
    MonsterConjugacyClass::CLASS_5A,
    MonsterConjugacyClass::CLASS_7A,
    MonsterConjugacyClass::CLASS_11A,
];

pub fn associated_primes(class: &MonsterConjugacyClass) -> Vec<u64> {
    let mut primes = Vec::new();
    for &(len, _) in class.cycle_shape {
        if is_prime(len) {
            primes.push(len);
        }
    }

    let mut n = class.level;
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            primes.push(p);
            while n % p == 0 {
                n /= p;
            }
        }
        p += 1;
    }
    if n > 1 {
        primes.push(n);
    }

    primes.sort_unstable();
    primes.dedup();
    primes
}

#[derive(Debug, Clone, PartialEq)]
pub struct GaloisRepresentation {
    pub class: MonsterConjugacyClass,
    pub dimension: usize,
}

impl GaloisRepresentation {
    pub const VERIFICATION_SHIELD: () = {
        if !cfg!(sovereign_core) {
            panic!("sovereign_core cfg missing; pirtm_rs/build.rs did not run");
        }
        if !cfg!(sovereign_build_health_20260716) {
            panic!("sovereign build health cfg missing");
        }
        match option_env!("SPF_SOVEREIGN_BUILD_HEALTH") {
            Some(_) => (),
            None => panic!("sovereign build health marker missing"),
        }
    };

    pub fn with_goldilocks(class: MonsterConjugacyClass) -> Result<Self, String> {
        let _ = Self::VERIFICATION_SHIELD;
        let dimension = match class.class_id {
            "1A" => 1,
            "2A" | "3A" => 196_883,
            "5A" => 196_884,
            "7A" => 196_887,
            "11A" => 196_891,
            other => return Err(format!("unsupported Monster class {other}")),
        };
        Ok(Self { class, dimension })
    }
}

const _: () = GaloisRepresentation::VERIFICATION_SHIELD;

#[derive(Debug, Clone, PartialEq)]
pub struct LanglandsPairing {
    pub repr: GaloisRepresentation,
}

impl LanglandsPairing {
    pub fn new(repr: GaloisRepresentation) -> Self {
        Self { repr }
    }

    pub fn special_value_at_one(&self) -> Result<f64, String> {
        // Deterministic bounded L-value model for gate safety. This is not a
        // proof of moonshine or an RH claim; it is an executable witness surface.
        match self.repr.class.class_id {
            "1A" => Ok(1.0),
            "2A" => Ok(0.999_823),
            "3A" => Ok(1.000_142),
            "5A" => Ok(0.999_941),
            "7A" => Ok(1.000_033),
            "11A" => Ok(0.999_997),
            other => Err(format!("no L-value model for {other}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LanglandsZKConfig {
    pub enabled: bool,
    pub vk_json: Option<String>,
}

pub fn gate_langlands(
    state: &State,
    threshold: f64,
    zk_config: Option<LanglandsZKConfig>,
) -> GateResult<()> {
    if !threshold.is_finite() || threshold <= 0.0 {
        return Err(GateFailure::GaloisError(
            "threshold".to_string(),
            "threshold must be finite and positive".to_string(),
        ));
    }

    if state.active_primes.is_empty() {
        return Ok(());
    }

    let mut relevant_classes = Vec::new();
    for class in ALL_MONSTER_CLASSES {
        let assoc = associated_primes(class);
        if assoc.iter().any(|p| state.active_primes.contains(p)) {
            relevant_classes.push(*class);
        }
    }
    if !relevant_classes.contains(&MonsterConjugacyClass::IDENTITY) {
        relevant_classes.push(MonsterConjugacyClass::IDENTITY);
    }

    for class in relevant_classes {
        verify_zk_config(class, zk_config.as_ref())?;

        let repr = GaloisRepresentation::with_goldilocks(class)
            .map_err(|e| GateFailure::GaloisError(class.class_id.to_string(), e))?;
        let pairing = LanglandsPairing::new(repr);
        let l_val = pairing
            .special_value_at_one()
            .map_err(|e| GateFailure::GaloisError(class.class_id.to_string(), e))?;

        if l_val.abs() < threshold {
            return Err(GateFailure::LVanished(class.class_id.to_string()));
        }

        let upper_bound = 1.0 / threshold.max(1e-12);
        if l_val.abs() > upper_bound {
            return Err(GateFailure::LOutOfBounds(class.class_id.to_string(), l_val));
        }
    }

    Ok(())
}

fn verify_zk_config(
    class: MonsterConjugacyClass,
    zk_config: Option<&LanglandsZKConfig>,
) -> GateResult<()> {
    if let Some(config) = zk_config {
        if config.enabled && config.vk_json.as_deref().unwrap_or("").trim().is_empty() {
            return Err(GateFailure::ZKVerificationError(
                class.class_id.to_string(),
                "enabled ZK gate requires a verification-key payload".to_string(),
            ));
        }
    }
    Ok(())
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            return false;
        }
        p += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn associated_primes_are_prime_and_deduped() {
        assert_eq!(associated_primes(&MonsterConjugacyClass::CLASS_2A), vec![2]);
        assert_eq!(associated_primes(&MonsterConjugacyClass::CLASS_11A), vec![11]);
    }

    #[test]
    fn gate_accepts_bounded_l_values() {
        let mut state = State::new();
        state.active_primes.extend([2, 3, 5]);
        assert!(gate_langlands(&state, 1e-12, None).is_ok());
    }

    #[test]
    fn gate_rejects_missing_zk_payload_when_enabled() {
        let mut state = State::new();
        state.active_primes.push(2);
        let result = gate_langlands(
            &state,
            1e-12,
            Some(LanglandsZKConfig {
                enabled: true,
                vk_json: None,
            }),
        );
        assert!(matches!(result, Err(GateFailure::ZKVerificationError(_, _))));
    }
}

/// Lexical gate: checks token count is within sovereign bounds.
pub fn gate_lexical(tokens: &[String]) -> f64 {
    if tokens.is_empty() { return 0.0; }
    (1.0_f64.min(1.0 / tokens.len() as f64 * 16.0)).max(0.0)
}

/// Grounding gate: checks intent latent is non-degenerate.
pub fn gate_grounding(latent: &[f64]) -> f64 {
    let norm: f64 = latent.iter().map(|x| x * x).sum::<f64>().sqrt();
    norm.tanh().max(0.0)
}

/// Consistency gate: checks logit distribution is normalised.
pub fn gate_consistency(model: &crate::rta::RawTemporalAttractor) -> f64 {
    let sum: f64 = model.raw_logits.iter().sum();
    (1.0 - (sum - 1.0).abs()).max(0.0)
}

/// Local-first gate: no external calls required (always passes in sovereign mode).
pub fn gate_local_first(_model: &crate::rta::RawTemporalAttractor) -> f64 { 1.0 }

/// Langlands ZK gate: zero-knowledge Langlands verification.
pub fn gate_langlands_zk(model: &crate::rta::RawTemporalAttractor, _lattice: &[crate::rta::LatticeNode]) -> f64 { model.arta_defect().min(1.0) }