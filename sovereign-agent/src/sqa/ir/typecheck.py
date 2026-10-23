# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Type / resource checks for QuantumProgram."""

from __future__ import annotations

from .schema import (
    QuantumProgram, TypeCheckError, ResourceError, SUPPORTED_GATES, IR_VERSION,
)


def _build_qreg_map(program: QuantumProgram) -> dict[str, int]:
    return {r.name: r.size for r in program.qregs}


def _build_creg_map(program: QuantumProgram) -> dict[str, int]:
    return {r.name: r.size for r in program.cregs}


def _check_qubit_ref(ref, qmap: dict[str, int], context: str) -> None:
    if ref.register not in qmap:
        raise TypeCheckError(f"{context}: unknown qreg {ref.register}")
    if not 0 <= ref.index < qmap[ref.register]:
        raise TypeCheckError(f"{context}: qubit index out of range")


def _check_classical_ref(ref, cmap: dict[str, int], context: str) -> None:
    if ref.register not in cmap:
        raise TypeCheckError(f"{context}: unknown creg {ref.register}")
    if not 0 <= ref.index < cmap[ref.register]:
        raise TypeCheckError(f"{context}: cbit index out of range")


def _arity(gate: str) -> int:
    if gate in ("H", "X", "Z", "MEASURE"):
        return 1
    if gate == "CNOT":
        return 2
    raise TypeCheckError(f"unsupported gate {gate}")


def type_check_program(program: QuantumProgram) -> None:
    if program.ir_version != IR_VERSION:
        raise TypeCheckError(f"unsupported ir_version {program.ir_version}")
    qmap = _build_qreg_map(program)
    cmap = _build_creg_map(program)
    total_q = sum(qmap.values())
    if program.resources and total_q > program.resources.max_qubits:
        raise ResourceError("exceeds max_qubits")
    if program.resources and len(program.operations) > program.resources.max_gates:
        raise ResourceError("exceeds max_gates")
    for i, op in enumerate(program.operations):
        ctx = f"op[{i}] {op.gate}"
        if op.gate not in SUPPORTED_GATES and op.gate != "Z":
            raise TypeCheckError(f"{ctx}: unsupported gate")
        if len(op.qubits) != _arity(op.gate):
            raise TypeCheckError(f"{ctx}: wrong arity")
        for q in op.qubits:
            _check_qubit_ref(q, qmap, ctx)
        if op.gate == "MEASURE":
            if op.classical is None:
                raise TypeCheckError(f"{ctx}: MEASURE requires classical target")
            _check_classical_ref(op.classical, cmap, ctx)


def is_well_typed(program: QuantumProgram) -> bool:
    try:
        type_check_program(program)
        return True
    except (TypeCheckError, ResourceError):
        return False
