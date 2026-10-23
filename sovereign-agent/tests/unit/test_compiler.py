# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Unit tests for circuit compiler."""

from __future__ import annotations

from sqa.ir.parser import parse_and_typecheck
from sqa.compiler.circuit import compile_program

PLUS = {
    "ir_version": "0.1.0",
    "qregs": [{"name": "q", "size": 1}],
    "cregs": [{"name": "c", "size": 1}],
    "operations": [
        {"gate": "H", "qubits": [{"register": "q", "index": 0}]},
        {"gate": "MEASURE", "qubits": [{"register": "q", "index": 0}],
         "classical": {"register": "c", "index": 0}},
    ],
}


def test_compile_plus():
    prog = parse_and_typecheck(PLUS)
    cc = compile_program(prog)
    assert cc.n_qubits == 1
    assert cc.artifact_hash()
