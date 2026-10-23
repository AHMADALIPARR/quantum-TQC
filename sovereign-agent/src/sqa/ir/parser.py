# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Parse / serialize QuantumProgram IR."""

from __future__ import annotations

import json
import hashlib
from pathlib import Path
from typing import Any

from .schema import (
    QuantumProgram, QuantumRegister, ClassicalRegister, GateOp,
    QubitRef, ClassicalBitRef, ResourceConstraints, AlgorithmMetadata,
    BackendRequirements, SchemaError, IR_VERSION,
)
from .typecheck import type_check_program


def _qubit(d: dict) -> QubitRef:
    return QubitRef(register=d["register"], index=int(d["index"]))


def _cbit(d: dict) -> ClassicalBitRef:
    return ClassicalBitRef(register=d["register"], index=int(d["index"]))


def parse_program(data: dict | str | bytes) -> QuantumProgram:
    if isinstance(data, (str, bytes)):
        data = json.loads(data)
    if not isinstance(data, dict):
        raise SchemaError("IR must be a JSON object")
    ver = data.get("ir_version", IR_VERSION)
    qregs = [QuantumRegister(**r) for r in data.get("qregs", [])]
    cregs = [ClassicalRegister(**r) for r in data.get("cregs", [])]
    ops = []
    for o in data.get("operations", []):
        qubits = [_qubit(q) for q in o.get("qubits", [])]
        classical = _cbit(o["classical"]) if o.get("classical") else None
        ops.append(GateOp(gate=o["gate"], qubits=qubits, classical=classical, source_id=o.get("source_id")))
    meta = data.get("metadata")
    metadata = AlgorithmMetadata(**meta) if meta else None
    res = data.get("resources")
    resources = ResourceConstraints(**res) if res else None
    br = data.get("backend_requirements")
    backend_requirements = BackendRequirements(**br) if br else None
    return QuantumProgram(
        ir_version=ver, qregs=qregs, cregs=cregs, operations=ops,
        metadata=metadata, resources=resources, backend_requirements=backend_requirements,
    )


def parse_and_typecheck(data: dict | str | bytes) -> QuantumProgram:
    prog = parse_program(data)
    type_check_program(prog)
    return prog


def serialize_program(program: QuantumProgram, *, pretty: bool = False) -> str:
    return json.dumps(program.to_dict(), indent=2 if pretty else None, sort_keys=True)


def program_hash(program: QuantumProgram) -> str:
    raw = serialize_program(program, pretty=False)
    return hashlib.sha256(raw.encode()).hexdigest()


def load_program_file(path: str) -> QuantumProgram:
    return parse_and_typecheck(Path(path).read_text())


def save_program_file(program: QuantumProgram, path: str, *, pretty: bool = True) -> None:
    Path(path).write_text(serialize_program(program, pretty=pretty))
