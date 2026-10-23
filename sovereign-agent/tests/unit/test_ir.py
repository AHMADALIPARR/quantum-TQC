# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Unit tests for IR parse/typecheck."""

from __future__ import annotations

from sqa.ir.parser import parse_and_typecheck, program_hash
from sqa.ir.schema import TypeCheckError
import pytest

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


def test_parse_plus():
    prog = parse_and_typecheck(PLUS)
    assert len(prog.operations) == 2
    assert program_hash(prog)


def test_bad_qubit_index():
    bad = dict(PLUS)
    bad["operations"] = [{"gate": "H", "qubits": [{"register": "q", "index": 9}]}]
    with pytest.raises(TypeCheckError):
        parse_and_typecheck(bad)
