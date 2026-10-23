<!--
Copyright (C) 2026
Licensed under the GNU Affero General Public License v3.0.
See LICENSE for the full text.
-->

<div align="center">

# Quantum TQC

### Topological Quantum Computing · CPSC/QFT · Null-Collapse

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Q#](https://img.shields.io/badge/Q%23-null%20collapse-purple)](null-collapse-tqc/)
[![Lean](https://img.shields.io/badge/Lean-formal-green)](cpsc-qft/lean/)
[![Rust](https://img.shields.io/badge/Rust-CPSC%2FQFT-orange)](cpsc-qft/rust/)

**Source-preserved frameworks under AGPL-3.0.**

</div>

---

## Components

| Tree | Role |
|------|------|
| **[`cpsc-qft/`](cpsc-qft/)** | CPSC / QFT: Rust (Roswell metric, Dodleston anomaly, φ⁴ encoding, leakage bounds), Lean Mathlib instantiation, OpenQASM φ⁴ Trotter, OWL Planck thermo |
| **[`null-collapse-tqc/`](null-collapse-tqc/)** | Null-collapse topological QC: Q# multi-pair null collapse, Q# tests, Lean topological invariance sketch, algorithm spec |

Languages: **Rust, Q#, Lean, OpenQASM, OWL**. No Python.

All source blocks are retained as supplied (including placeholders and incomplete definitions). Packaging does not assert build, proof, or experimental validation.

---

## Layout

```
quantum-TQC/
├── LICENSE                          # GNU AGPL v3.0
├── README.md
├── cpsc-qft/
│   ├── lean/MathlibInstantiation.lean
│   ├── owl/planck_thermo.owl
│   ├── qasm/phi4_trotter.qasm
│   ├── rust/
│   ├── PACKAGING_NOTE.txt
│   └── SHA256SUMS
└── null-collapse-tqc/
    ├── formal/TopologicalInvariance.lean
    ├── spec/algorithm.txt
    ├── src/NullCollapseMulti.qs
    ├── tests/NullCollapseTests.qs
    ├── README.md
    └── SHA256SUMS.json
```

---

## Quick links

### CPSC / QFT
- [Rust sources](cpsc-qft/rust/)
- [Lean Mathlib instantiation](cpsc-qft/lean/MathlibInstantiation.lean)
- [φ⁴ Trotter QASM](cpsc-qft/qasm/phi4_trotter.qasm)
- [Planck thermo OWL](cpsc-qft/owl/planck_thermo.owl)

### Null-collapse TQC
- [NullCollapseMulti.qs](null-collapse-tqc/src/NullCollapseMulti.qs)
- [NullCollapseTests.qs](null-collapse-tqc/tests/NullCollapseTests.qs)
- [TopologicalInvariance.lean](null-collapse-tqc/formal/TopologicalInvariance.lean)
- [Algorithm spec](null-collapse-tqc/spec/algorithm.txt)

---

## How the pieces connect

1. **Null-collapse TQC** — Q# topological null-collapse primitive and Lean invariance sketch.
2. **CPSC/QFT** — continuum / field encodings (φ⁴, leakage, Roswell/Dodleston) plus OWL/QASM.

Shared AGPL-3.0 license. No automated integration harness is claimed in this packaging.

---

## License

**GNU Affero General Public License v3.0** — see [LICENSE](LICENSE).

---

<div align="center">

**Quantum TQC** · AGPL-3.0 · Rust · Q# · Lean · QASM · OWL

</div>
