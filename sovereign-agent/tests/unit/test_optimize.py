# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Unit tests for decompose / optimize placeholders."""

from __future__ import annotations

from sqa.compiler.decompose import decompose_gates
from sqa.ir.schema import GateOp, QubitRef


def test_decompose_identity():
    ops = [GateOp(gate="H", qubits=[QubitRef(register="q", index=0)])]
    assert len(decompose_gates(ops)) == 1
