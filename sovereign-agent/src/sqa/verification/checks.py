# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Independent verification checks."""

from __future__ import annotations

from typing import List, Tuple
from sqa.ir.schema import QuantumProgram, SUPPORTED_GATES
from sqa.compiler.circuit import CompiledCircuit


def run_checks(program: QuantumProgram, compiled: CompiledCircuit) -> Tuple[bool, List[str]]:
    msgs: List[str] = []
    ok = True
    if compiled.n_qubits <= 0:
        ok = False
        msgs.append("compiled circuit has no qubits")
    if len(compiled.ops) != len(program.operations):
        msgs.append("op count mismatch (warning)")
    for op in compiled.ops:
        if op.gate not in SUPPORTED_GATES and op.gate != "Z":
            ok = False
            msgs.append(f"unsupported compiled gate {op.gate}")
        for q in op.qubits:
            if not 0 <= q < compiled.n_qubits:
                ok = False
                msgs.append(f"qubit index {q} out of range")
    if not msgs:
        msgs.append("all checks passed")
    return ok, msgs
