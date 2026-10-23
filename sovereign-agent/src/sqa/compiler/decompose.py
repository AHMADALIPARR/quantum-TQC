# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Gate decomposition passes."""

from __future__ import annotations

from typing import List
from sqa.ir.schema import GateOp


def decompose_gates(ops: List[GateOp]) -> List[GateOp]:
    """Identity pass for native {H,X,Z,CNOT,MEASURE}; extend later."""
    return list(ops)
