# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Unit tests for Fibonacci anyon model."""

from __future__ import annotations

from sqa.topological.fibonacci import (
    Anyon, fusion_outcomes, quantum_dimension, verify_pentagon_hexagon, PHI_F,
)
from sqa.topological.braid import fusion_space_dim, fibonacci


def test_fusion_tau_tau():
    outs = fusion_outcomes(Anyon.TAU, Anyon.TAU)
    assert Anyon.VACUUM in outs and Anyon.TAU in outs


def test_quantum_dimension():
    assert float(quantum_dimension(Anyon.TAU).evalf()) == PHI_F


def test_pentagon_hexagon():
    ok, msgs = verify_pentagon_hexagon()
    assert ok, msgs


def test_fusion_space_dim():
    assert fusion_space_dim(3, Anyon.TAU) == fibonacci(3)
