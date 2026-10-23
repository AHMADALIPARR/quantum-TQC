# Architecture — Milestone 1 Phase 1

## Trust Boundaries

```
[Agent / User]  →  Typed IR (JSON)  →  Type Checker  →  Circuit Compiler
                                                              │
                                                              ▼
                                                       CompiledCircuit
                                                       (hashable artifact)
```

The agent may only emit IR. The host enforces schema, typing, and resource limits.
Compilation is deterministic and produces a content-addressed artifact.

## IR Version Compatibility

- Current: `0.1.0`
- Future versions must be explicitly accepted; unknown versions are rejected.

## Artifact Integrity

- `program_hash(program)` = SHA-256 of canonical JSON serialization.
- `CompiledCircuit.artifact_hash()` likewise.
- Source maps preserve IR → compiled operation linkage.

## Next Phase

1. Independent Verification engine (mandatory checks before any backend call).
2. QuTiP state-vector execution backend.
3. Signed execution receipts.
4. State machine enforcement (CREATED → … → SUCCEEDED/FAILED).
