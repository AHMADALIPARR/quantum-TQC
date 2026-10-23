# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0

"""Unit tests for oracle interface and linear Boolean oracles."""

from __future__ import annotations

import pytest
from sqa.oracle import LinearBooleanOracle, BooleanOracle


class TestLinearBooleanOracle:
    def test_parity_2bit(self):
        ora = LinearBooleanOracle("parity2", 2, mask=0b11)
        assert ora.classical_eval([0, 0]) == [0]
        assert ora.classical_eval([0, 1]) == [1]
        assert ora.classical_eval([1, 0]) == [1]
        assert ora.classical_eval([1, 1]) == [0]
        ok, msgs = ora.verify_classical_truth_table()
        assert ok, msgs

    def test_synthesize_cnot_count(self):
        ora = LinearBooleanOracle("x0", 3, mask=0b001)
        ops = ora.synthesize("q", "out")
        assert len(ops) == 1
        assert ops[0].gate == "CNOT"

    def test_contract(self):
        ora = LinearBooleanOracle("parity", 4, mask=0b1111)
        c = ora.contract()
        assert c.input_bits == 4
        assert c.output_bits == 1
        assert c.reversible is True
