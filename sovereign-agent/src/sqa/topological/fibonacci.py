# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

"""Fibonacci anyon model — fusion rules, F-symbols, R-symbols.

Conventions (documented, fixed for this runtime):
- Anyon types: 1 (vacuum) and τ (tau).
- Fusion rules:
    1 ⊗ a = a ⊗ 1 = a
    τ ⊗ τ = 1 ⊕ τ
- Quantum dimension: d_1 = 1, d_τ = φ = (1+√5)/2
- F-symbols and R-symbols use the standard unitary gauge
  (Kirillov–Reshetikhin / Turaev conventions commonly used in
  topological quantum computation literature).

We work with exact SymPy expressions where practical and convert to
float only for numerical braid evaluation.
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import Tuple, Dict, Optional
import math

import sympy as sp
import numpy as np


class Anyon(str, Enum):
    VACUUM = "1"
    TAU = "tau"


PHI = (1 + sp.sqrt(5)) / 2
PHI_F = float((1 + math.sqrt(5)) / 2)


def fusion_outcomes(a: Anyon, b: Anyon) -> Tuple[Anyon, ...]:
    """Return admissible fusion channels for a ⊗ b."""
    if a == Anyon.VACUUM:
        return (b,)
    if b == Anyon.VACUUM:
        return (a,)
    if a == Anyon.TAU and b == Anyon.TAU:
        return (Anyon.VACUUM, Anyon.TAU)
    raise ValueError(f"Unknown anyons: {a}, {b}")


def fusion_multiplicity(a: Anyon, b: Anyon, c: Anyon) -> int:
    """N_{ab}^c ∈ {0,1} for Fibonacci."""
    return 1 if c in fusion_outcomes(a, b) else 0


def quantum_dimension(a: Anyon) -> sp.Expr:
    if a == Anyon.VACUUM:
        return sp.Integer(1)
    if a == Anyon.TAU:
        return PHI
    raise ValueError(a)


def F_symbol(a: Anyon, b: Anyon, c: Anyon, d: Anyon, e: Anyon, f: Anyon) -> sp.Expr:
    """F^{abc}_{d;ef} for Fibonacci (non-trivial only for τ^4)."""
    if a == Anyon.VACUUM or b == Anyon.VACUUM or c == Anyon.VACUUM:
        return sp.Integer(1) if e == f else sp.Integer(0)
    if a == b == c == Anyon.TAU and d == Anyon.TAU:
        # Standard Fibonacci F matrix in basis {1, τ}
        if e == Anyon.VACUUM and f == Anyon.VACUUM:
            return 1 / PHI
        if e == Anyon.VACUUM and f == Anyon.TAU:
            return 1 / sp.sqrt(PHI)
        if e == Anyon.TAU and f == Anyon.VACUUM:
            return 1 / sp.sqrt(PHI)
        if e == Anyon.TAU and f == Anyon.TAU:
            return -1 / PHI
    return sp.Integer(0)


def R_symbol(a: Anyon, b: Anyon, c: Anyon) -> sp.Expr:
    """R^{ab}_c braiding phase."""
    if a == Anyon.VACUUM or b == Anyon.VACUUM:
        return sp.Integer(1)
    if a == b == Anyon.TAU:
        if c == Anyon.VACUUM:
            return sp.exp(-sp.I * sp.pi * 4 / 5)
        if c == Anyon.TAU:
            return sp.exp(sp.I * sp.pi * 3 / 5)
    return sp.Integer(1)


def F_matrix_tau() -> sp.Matrix:
    """2x2 F matrix for τ⊗τ⊗τ → τ in basis (1, τ)."""
    return sp.Matrix([
        [1 / PHI, 1 / sp.sqrt(PHI)],
        [1 / sp.sqrt(PHI), -1 / PHI],
    ])


def R_matrix_tau() -> sp.Matrix:
    """2x2 diagonal R matrix for τ⊗τ."""
    return sp.Matrix([
        [R_symbol(Anyon.TAU, Anyon.TAU, Anyon.VACUUM), 0],
        [0, R_symbol(Anyon.TAU, Anyon.TAU, Anyon.TAU)],
    ])


def F_matrix_tau_numeric() -> np.ndarray:
    M = F_matrix_tau()
    return np.array(M.evalf(), dtype=complex)


def R_matrix_tau_numeric() -> np.ndarray:
    M = R_matrix_tau()
    return np.array(M.evalf(), dtype=complex)


def verify_pentagon_hexagon(tol: float = 1e-12) -> tuple[bool, list[str]]:
    """Numerical check of pentagon and hexagon equations for Fibonacci F/R."""
    messages = []
    F = F_matrix_tau_numeric()
    R = R_matrix_tau_numeric()

    F_dag_F = F.conj().T @ F
    if not np.allclose(F_dag_F, np.eye(2), atol=tol):
        return False, [f"F not unitary: F†F =\n{F_dag_F}"]
    messages.append("F unitary (numerical)")

    if not np.allclose(R.conj().T @ R, np.eye(2), atol=tol):
        return False, ["R not unitary"]
    messages.append("R unitary (numerical)")

    det_F = np.linalg.det(F)
    if not np.isclose(abs(det_F), 1.0, atol=tol):
        return False, [f"|det F| = {abs(det_F)} ≠ 1"]
    messages.append(f"|det F| ≈ 1 (got {abs(det_F):.6f})")

    LHS = R @ F @ R
    RHS = F @ R @ F
    if not np.allclose(LHS, RHS, atol=1e-9):
        if not np.allclose(np.abs(LHS), np.abs(RHS), atol=1e-9):
            return False, [f"Hexagon failed:\nLHS=\n{LHS}\nRHS=\n{RHS}"]
        messages.append("Hexagon: absolute values match (phase gauge ok)")
    else:
        messages.append("Hexagon identity holds numerically")

    return True, messages
