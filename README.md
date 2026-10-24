<!--
Copyright (C) 2026
Licensed under the GNU Affero General Public License v3.0.
See LICENSE for the full text.
-->

<div align="center">

# Quantum TQC

### Topological Quantum Computing · CPSC/QFT · Null-Collapse · Quantum Agent

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Q#](https://img.shields.io/badge/Q%23-null%20collapse-purple)](null-collapse-tqc/)
[![Lean](https://img.shields.io/badge/Lean-formal-green)](cpsc-qft/lean/)
[![Rust](https://img.shields.io/badge/Rust-CPSC%2FQFT-orange)](cpsc-qft/rust/)
[![OpenQASM](https://img.shields.io/badge/OpenQASM-circuits-red)](quantum-agent/circuits/)
[![TOML](https://img.shields.io/badge/TOML-agent%20schema-blue)](quantum-agent/)

**Three source frameworks under AGPL-3.0. Algorithms execute as OpenQASM circuits only.**

</div>

---

## Source inventory (all 3 zips)

### 1. `cpsc-qft-framework` → [`cpsc-qft/`](cpsc-qft/)

| Path | Present |
|------|---------|
| `rust/Phi4Encoding.rs` | yes |
| `rust/dodleston_anomaly.rs` | yes |
| `rust/leakage_bound.rs` | yes |
| `rust/phi4_oscillator_alternative.rs` | yes |
| `rust/roswell_metric.rs` | yes (full Kretschmann) |
| `lean/MathlibInstantiation.lean` | yes |
| `lean/roswell_anomaly.lean` | yes |
| `qasm/phi4_trotter.qasm` | yes |
| `owl/planck_thermo.owl` | yes |
| `fdtd_bus/` (6502 bus coupling Γ) | yes |
| `PACKAGING_NOTE.txt`, `SHA256SUMS` | yes |

### 2. `null-collapse-tqc-framework` → [`null-collapse-tqc/`](null-collapse-tqc/)

| Path | Present |
|------|---------|
| `src/NullCollapseMulti.qs` | yes |
| `tests/NullCollapseTests.qs` | yes |
| `formal/TopologicalInvariance.lean` | yes |
| `spec/algorithm.txt` | yes |
| `README.md`, `SHA256SUMS.json` | yes |
| `circuits/*.qasm` (full circuit expansion) | yes |

### 3. `sovereign-quantum-agent-agpl` → [`quantum-agent/`](quantum-agent/) (TOML + OpenQASM, no Python)

| Field | Location |
|-------|----------|
| IR schema | `ir/schema.toml` |
| Runtime state machine | `runtime/state_machine.toml` |
| Algorithms catalog | `algorithms/catalog.toml` |
| Fibonacci topological | `topological/fibonacci.toml` |
| Verification checks | `verification/checks.toml` |
| Receipts | `receipts/schema.toml` |
| Architecture / capability | `docs/*.toml` |
| **Executable circuits** | `circuits/*.qasm` |

---

## Algorithms (circuit-only)

All runnable algorithms are **fully expanded OpenQASM 2.0**. No host language.

### 1. Plus state — [`quantum-agent/circuits/plus_state.qasm`](quantum-agent/circuits/plus_state.qasm)

```qasm
OPENQASM 2.0;
include "qelib1.inc";
qreg q[1];
creg c[1];
h q[0];
measure q[0] -> c[0];
```

### 2. Bell pair — [`quantum-agent/circuits/bell_pair.qasm`](quantum-agent/circuits/bell_pair.qasm)

```qasm
OPENQASM 2.0;
include "qelib1.inc";
qreg q[2];
creg c[2];
h q[0];
cx q[0], q[1];
measure q[0] -> c[0];
measure q[1] -> c[1];
```

### 3. Grover n=2, marked \|01⟩ — [`quantum-agent/circuits/grover_n2_marked1.qasm`](quantum-agent/circuits/grover_n2_marked1.qasm)

```qasm
OPENQASM 2.0;
include "qelib1.inc";
qreg q[2];
creg c[2];
// superposition
h q[0]; h q[1];
// oracle |01>
x q[0]; h q[1]; cx q[0], q[1]; h q[1]; x q[0];
// diffusion
h q[0]; h q[1]; x q[0]; x q[1];
h q[1]; cx q[0], q[1]; h q[1];
x q[0]; x q[1]; h q[0]; h q[1];
measure q[0] -> c[0];
measure q[1] -> c[1];
```

### 4. Grover n=2, marked \|11⟩ — [`quantum-agent/circuits/grover_n2_marked3.qasm`](quantum-agent/circuits/grover_n2_marked3.qasm)

```qasm
OPENQASM 2.0;
include "qelib1.inc";
qreg q[2];
creg c[2];
h q[0]; h q[1];
h q[1]; cx q[0], q[1]; h q[1];          // oracle |11>
h q[0]; h q[1]; x q[0]; x q[1];
h q[1]; cx q[0], q[1]; h q[1];
x q[0]; x q[1]; h q[0]; h q[1];
measure q[0] -> c[0];
measure q[1] -> c[1];
```

### 5. Full Grover catalog, n ≤ 4

One marked item, `k = floor(π/4 √(2^n))` iterates, q[0] = leftmost ket bit.
Files: `quantum-agent/circuits/grover_n{n}_marked{m}.qasm` (30 circuits).
n=4 uses one clean ancilla `q[4]`.

| n | k | Success probability |
|---|---|---------------------|
| 1 | 1 | 1/2 |
| 2 | 1 | 1 |
| 3 | 2 | 121/128 |
| 4 | 3 | ≈ 0.9613 |

Record: [`quantum-agent/verification/circuit_checks.toml`](quantum-agent/verification/circuit_checks.toml).

### 6. Fibonacci braids — [`quantum-agent/circuits/topological/`](quantum-agent/circuits/topological/)

`u3` circuits of the F and R matrices in `fibonacci.toml`, plus braid words
`[0,1,0]` and `[0,1,0,1]`. The word the skeleton labeled exact logical X
is not Pauli X under those matrices (average gate fidelity 0.745 to X,
0.991 to H). The circuit is the braid, and the error is in the file header.

### 7. Null-collapse (local, m=3) — [`null-collapse-tqc/circuits/null_collapse_local_m3.qasm`](null-collapse-tqc/circuits/null_collapse_local_m3.qasm)

From `spec/algorithm.txt`: θ = 2 arcsin(w), Ry, Z braid, measure; collapsed = (c == 0).

```qasm
OPENQASM 2.0;
include "qelib1.inc";
qreg q[3];
creg c[3];
ry(1.0471975512) q[0];   // w=0.5
ry(0.6082466944) q[1];   // w=0.3
ry(1.8545904360) q[2];   // w=0.8
z q[0]; z q[1]; z q[2];  // braid σ² = R1(π)
measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];
```

### 8. Null-collapse (pairwise braid) — [`null-collapse-tqc/circuits/null_collapse_pair_m3.qasm`](null-collapse-tqc/circuits/null_collapse_pair_m3.qasm)

```qasm
OPENQASM 2.0;
include "qelib1.inc";
qreg q[3];
creg c[3];
ry(1.0471975512) q[0];
ry(0.6082466944) q[1];
ry(1.8545904360) q[2];
z q[0]; z q[1]; z q[2];
cx q[0], q[1]; h q[1]; cx q[0], q[1];
cx q[1], q[2]; h q[2]; cx q[1], q[2];
measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];
```

### 9. φ⁴ Trotter (CPSC/QFT) — [`cpsc-qft/qasm/phi4_trotter.qasm`](cpsc-qft/qasm/phi4_trotter.qasm)

OpenQASM 3.0 φ⁴ interaction step (Λ=8 → 3 qubits). See source for Trotter structure.

---

## Layout

```
quantum-TQC/
├── LICENSE
├── README.md
├── cpsc-qft/                 # zip 1: Rust · Lean · QASM · OWL · FDTD bus
├── null-collapse-tqc/        # zip 2: Q# · Lean · circuits/
└── quantum-agent/            # zip 3 → TOML schema + circuits/
    ├── framework.toml
    ├── ir/ runtime/ algorithms/ topological/
    ├── verification/ receipts/ docs/ examples/
    └── circuits/             # executable OpenQASM only
```

---


## pirtm_rs

Langlands safety gates and the Raw Temporal Attractor types they call.

```
pirtm_rs/
├── build.rs          # sovereign_core + build-health cfgs the shield requires
├── src/gates.rs      # gate_langlands, lexical, grounding, consistency, local-first, ZK
└── src/rta.rs        # State, RawTemporalAttractor, LatticeNode
```

`cargo test` in `pirtm_rs/` — 3 gate tests, all passing.
The supplied `distance` paste ended at `.sqrt()`; only the closing braces were added.

## License

**GNU Affero General Public License v3.0** — see [LICENSE](LICENSE).

---

<div align="center">

**Quantum TQC** · AGPL-3.0 · Rust · Q# · Lean · OpenQASM · OWL · TOML

</div>
