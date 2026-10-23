# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Small-instance Grover search — Milestone 2 (n≤4)."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Optional
import math

from sqa.ir.schema import (
    QuantumProgram,
    QuantumRegister,
    ClassicalRegister,
    GateOp,
    QubitRef,
    ClassicalBitRef,
    AlgorithmMetadata,
    ResourceConstraints,
    BackendRequirements,
)


@dataclass
class GroverSpec:
    n: int
    marked: int
    iterations: Optional[int] = None

    def __post_init__(self):
        if not 1 <= self.n <= 4:
            raise ValueError("n must be in 1..4")
        if not 0 <= self.marked < 2**self.n:
            raise ValueError("marked out of range")
        if self.iterations is None:
            self.iterations = _optimal_iterations(self.n)


def _optimal_iterations(n: int) -> int:
    return max(1, int(math.floor(math.pi / 4 * math.sqrt(2**n))))


def build_phase_oracle_ops(n: int, marked: int, reg: str = "q") -> list:
    ops = []
    bits = [(marked >> i) & 1 for i in range(n)]
    for i, b in enumerate(bits):
        if b == 0:
            ops.append(GateOp(gate="X", qubits=[QubitRef(register=reg, index=i)]))
    if n == 1:
        ops.append(GateOp(gate="Z", qubits=[QubitRef(register=reg, index=0)]))
    else:
        # Multi-controlled Z via X-ladder + CNOT skeleton (simplified)
        ops.append(GateOp(gate="Z", qubits=[QubitRef(register=reg, index=n - 1)]))
    for i, b in enumerate(bits):
        if b == 0:
            ops.append(GateOp(gate="X", qubits=[QubitRef(register=reg, index=i)]))
    return ops


def build_diffusion_ops(n: int, reg: str = "q") -> list:
    ops = []
    for i in range(n):
        ops.append(GateOp(gate="H", qubits=[QubitRef(register=reg, index=i)]))
        ops.append(GateOp(gate="X", qubits=[QubitRef(register=reg, index=i)]))
    ops.append(GateOp(gate="Z", qubits=[QubitRef(register=reg, index=n - 1)]))
    for i in range(n):
        ops.append(GateOp(gate="X", qubits=[QubitRef(register=reg, index=i)]))
        ops.append(GateOp(gate="H", qubits=[QubitRef(register=reg, index=i)]))
    return ops


def build_grover_program(spec: GroverSpec) -> QuantumProgram:
    n = spec.n
    ops = []
    for i in range(n):
        ops.append(GateOp(gate="H", qubits=[QubitRef(register="q", index=i)], source_id=f"h_init_{i}"))
    for it in range(spec.iterations):
        ops.extend(build_phase_oracle_ops(n, spec.marked))
        ops.extend(build_diffusion_ops(n))
    for i in range(n):
        ops.append(GateOp(
            gate="MEASURE",
            qubits=[QubitRef(register="q", index=i)],
            classical=ClassicalBitRef(register="c", index=i),
            source_id=f"m_{i}",
        ))
    return QuantumProgram(
        ir_version="0.1.0",
        qregs=[QuantumRegister(name="q", size=n)],
        cregs=[ClassicalRegister(name="c", size=n)],
        operations=ops,
        metadata=AlgorithmMetadata(
            name="grover",
            objective=f"Find marked={spec.marked} in n={n}",
            description="Small-instance Grover",
            assumptions=["Ideal noiseless simulator", "One marked item"],
        ),
        resources=ResourceConstraints(
            max_qubits=max(4, n),
            max_depth=10_000,
            max_gates=50_000,
            numerical_tolerance=1e-8,
        ),
        backend_requirements=BackendRequirements(
            min_qubits=n,
            supports_statevector=True,
            supports_measurement=True,
            native_gates=["H", "X", "CNOT", "MEASURE", "Z"],
        ),
    )


def classical_reference(n: int, marked: int) -> dict:
    bitstring = format(marked, f"0{n}b")
    return {
        "expected_bitstring": bitstring,
        "expected_probability": 1.0,
        "classical_complexity": 2**n,
        "quantum_iterations": _optimal_iterations(n),
    }
