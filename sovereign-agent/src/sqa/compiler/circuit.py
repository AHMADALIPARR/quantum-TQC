# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Compile QuantumProgram to content-addressed CompiledCircuit."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import List, Optional
import hashlib
import json

from sqa.ir.schema import QuantumProgram, GateOp


@dataclass
class CompiledOp:
    gate: str
    qubits: List[int]  # global qubit indices
    classical: Optional[int] = None
    source_id: Optional[str] = None


@dataclass
class CompiledCircuit:
    n_qubits: int
    n_bits: int
    ops: List[CompiledOp]
    program_hash: str
    source_map: List[Optional[str]] = field(default_factory=list)

    def artifact_hash(self) -> str:
        payload = {
            "n_qubits": self.n_qubits,
            "n_bits": self.n_bits,
            "ops": [{"g": o.gate, "q": o.qubits, "c": o.classical} for o in self.ops],
            "program_hash": self.program_hash,
        }
        return hashlib.sha256(json.dumps(payload, sort_keys=True).encode()).hexdigest()


def compile_program(program: QuantumProgram) -> CompiledCircuit:
    q_offsets = {}
    off = 0
    for r in program.qregs:
        q_offsets[r.name] = off
        off += r.size
    n_qubits = off
    c_offsets = {}
    off = 0
    for r in program.cregs:
        c_offsets[r.name] = off
        off += r.size
    n_bits = off

    from sqa.ir.parser import program_hash as ph
    ops = []
    source_map = []
    for op in program.operations:
        global_q = [q_offsets[q.register] + q.index for q in op.qubits]
        global_c = None
        if op.classical:
            global_c = c_offsets[op.classical.register] + op.classical.index
        ops.append(CompiledOp(gate=op.gate, qubits=global_q, classical=global_c, source_id=op.source_id))
        source_map.append(op.source_id)
    return CompiledCircuit(
        n_qubits=n_qubits, n_bits=n_bits, ops=ops,
        program_hash=ph(program), source_map=source_map,
    )
