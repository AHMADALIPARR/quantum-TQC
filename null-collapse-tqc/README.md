# Null-Collapse TQC

Ising anyon null-collapse over the Euclidean plane (Quantum Euclidean H_n).

## Algorithm

Encode candidate weight w as fusion-channel amplitude → apply topologically protected
search braid β = σ₁ σ₂ σ₁ → measure intermediate charge → outcome 0 = null sector.

Full specification: [spec/NullCollapseAlgorithm.md](spec/NullCollapseAlgorithm.md)

## Circuit-only execution (OpenQASM)

| Circuit | Description |
|---------|-------------|
| [circuits/null_collapse_search_braid.qasm](circuits/null_collapse_search_braid.qasm) | Single candidate, full SearchBraid |
| [circuits/null_collapse_multi_m3.qasm](circuits/null_collapse_multi_m3.qasm) | Three independent pairs |
| [circuits/null_collapse_local_m3.qasm](circuits/null_collapse_local_m3.qasm) | Local Z-braid variant |
| [circuits/null_collapse_pair_m3.qasm](circuits/null_collapse_pair_m3.qasm) | Pairwise Clifford variant |

## Q# host (optional)

- [src/NullCollapse.qs](src/NullCollapse.qs) — BraidSigma1/2, SearchBraid, NullCollapse, NullCollapseAll
- [tests/NullCollapseTests.qs](tests/NullCollapseTests.qs)

Legacy multi-pair source: [src/NullCollapseMulti.qs](src/NullCollapseMulti.qs)

## Formal

- [formal/TopologicalInvariance.lean](formal/TopologicalInvariance.lean)

## License

AGPL-3.0-or-later
