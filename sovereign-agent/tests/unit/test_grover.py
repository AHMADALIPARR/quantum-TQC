# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Unit tests for Grover builder."""

from __future__ import annotations

from sqa.algorithms.grover import GroverSpec, build_grover_program, classical_reference, _optimal_iterations


def test_optimal_iterations():
    assert _optimal_iterations(2) >= 1


def test_build_program():
    spec = GroverSpec(n=2, marked=1)
    prog = build_grover_program(spec)
    assert prog.qregs[0].size == 2
    ref = classical_reference(2, 1)
    assert ref["expected_bitstring"] == "01"
