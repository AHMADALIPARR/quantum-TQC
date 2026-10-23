# Quantum Agent (TOML framework)

From-scratch **declarative** quantum agent stack. Schema, policy, algorithms,
topological model, and example programs are all **TOML** — no Python host
is shipped in this repository.

## Layout

```
quantum-agent/
├── framework.toml           # root manifest + principles
├── ir/schema.toml           # typed IR
├── runtime/state_machine.toml
├── algorithms/catalog.toml
├── topological/fibonacci.toml
├── verification/checks.toml
├── receipts/schema.toml
├── examples/                # IR programs as TOML
│   ├── plus_state.toml
│   ├── bell_pair.toml
│   └── grover_n2.toml
└── docs/
    ├── architecture.toml
    └── capability_matrix.toml
```

## Flow

```
Agent emits IR (TOML)
        ↓
Host: parse → typecheck → compile → verify → execute → receipt
```

Hosts may be implemented in any language that consumes these files.
This tree is the specification.

## License

AGPL-3.0-or-later (see repository root `LICENSE`).
