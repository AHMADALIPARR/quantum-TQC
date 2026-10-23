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
[![TOML](https://img.shields.io/badge/TOML-quantum%20agent-blue)](quantum-agent/)

**Source-preserved frameworks under AGPL-3.0.**

</div>

---

## Components

| Tree | Role |
|------|------|
| **[`cpsc-qft/`](cpsc-qft/)** | CPSC / QFT: Rust, Lean, OpenQASM, OWL |
| **[`null-collapse-tqc/`](null-collapse-tqc/)** | Null-collapse TQC: Q#, Lean formal, algorithm spec |
| **[`quantum-agent/`](quantum-agent/)** | From-scratch **TOML** quantum agent framework (IR, runtime, algorithms, Fibonacci, verification, receipts) |

Languages: **Rust, Q#, Lean, OpenQASM, OWL, TOML**. No Python.

---

## Layout

```
quantum-TQC/
├── LICENSE
├── README.md
├── cpsc-qft/           # Rust · Lean · QASM · OWL
├── null-collapse-tqc/  # Q# · Lean
└── quantum-agent/      # TOML declarative agent
    ├── framework.toml
    ├── ir/
    ├── runtime/
    ├── algorithms/
    ├── topological/
    ├── verification/
    ├── receipts/
    ├── examples/
    └── docs/
```

---

## Quantum Agent (TOML)

Declarative agent: agent emits IR → host parse/typecheck/compile/verify/execute/receipt.

- [framework.toml](quantum-agent/framework.toml) — principles and module index
- [ir/schema.toml](quantum-agent/ir/schema.toml) — typed gates and resources
- [runtime/state_machine.toml](quantum-agent/runtime/state_machine.toml)
- [topological/fibonacci.toml](quantum-agent/topological/fibonacci.toml)
- [examples/](quantum-agent/examples/) — plus_state, bell_pair, grover_n2

No language host is shipped; TOML is the specification.

---

## Quick links

### CPSC / QFT
- [Rust](cpsc-qft/rust/) · [Lean](cpsc-qft/lean/MathlibInstantiation.lean) · [QASM](cpsc-qft/qasm/phi4_trotter.qasm) · [OWL](cpsc-qft/owl/planck_thermo.owl)

### Null-collapse TQC
- [NullCollapseMulti.qs](null-collapse-tqc/src/NullCollapseMulti.qs) · [Tests](null-collapse-tqc/tests/) · [Lean formal](null-collapse-tqc/formal/TopologicalInvariance.lean)

### Quantum Agent
- [README](quantum-agent/README.md) · [Architecture](quantum-agent/docs/architecture.toml) · [Capability matrix](quantum-agent/docs/capability_matrix.toml)

---

## License

**GNU Affero General Public License v3.0** — see [LICENSE](LICENSE).

---

<div align="center">

**Quantum TQC** · AGPL-3.0 · Rust · Q# · Lean · QASM · OWL · TOML

</div>
