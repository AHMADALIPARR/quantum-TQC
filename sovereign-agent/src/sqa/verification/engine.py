# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Verification engine — mandatory checks before backend execution."""

from __future__ import annotations

from typing import List, Tuple
from sqa.ir.schema import QuantumProgram
from sqa.compiler.circuit import CompiledCircuit
from .checks import run_checks


class VerificationEngine:
    def verify(self, program: QuantumProgram, compiled: CompiledCircuit) -> Tuple[bool, List[str]]:
        return run_checks(program, compiled)
