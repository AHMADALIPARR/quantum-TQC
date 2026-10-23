# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Braid generators and fusion-space representations for Fibonacci anyons."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import List, Tuple, Optional, Dict, Sequence
import numpy as np

from .fibonacci import (
    Anyon,
    fusion_outcomes,
    F_matrix_tau_numeric,
    R_matrix_tau_numeric,
    PHI_F,
)


def fibonacci(n: int) -> int:
    if n < 0:
        raise ValueError
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a


def fusion_space_dim(n_anyons: int, total: Anyon = Anyon.VACUUM) -> int:
    if n_anyons < 1:
        return 1 if total == Anyon.VACUUM else 0
    if total == Anyon.VACUUM:
        return fibonacci(n_anyons - 1) if n_anyons >= 1 else 1
    if total == Anyon.TAU:
        return fibonacci(n_anyons)
    return 0


def enumerate_fusion_basis(n_anyons: int, total: Anyon = Anyon.VACUUM) -> List[Tuple[Anyon, ...]]:
    if n_anyons == 0:
        return [()] if total == Anyon.VACUUM else []
    if n_anyons == 1:
        return [()] if total == Anyon.TAU else []
    bases: List[Tuple[Anyon, ...]] = []
    for c1 in fusion_outcomes(Anyon.TAU, Anyon.TAU):
        _extend([c1], 2, n_anyons, total, bases)
    return bases


def _extend(path: list, k: int, n: int, total: Anyon, out: list) -> None:
    if k == n:
        if path[-1] == total:
            out.append(tuple(path))
        return
    last = path[-1]
    for c in fusion_outcomes(last, Anyon.TAU):
        _extend(path + [c], k + 1, n, total, out)


@dataclass
class BraidGenerator:
    index: int
    inverse: bool = False
    n_strands: int = 3


def braid_generator_matrix(
    n_anyons: int,
    i: int,
    inverse: bool = False,
    total: Anyon = Anyon.VACUUM,
) -> np.ndarray:
    """Unitary matrix for braid generator σ_i on fusion space."""
    if n_anyons > 4:
        raise NotImplementedError("n>4 braid matrices not yet supported")
    return _braid_matrix_small(n_anyons, i, inverse, total)


def _braid_matrix_small(
    n: int, i: int, inverse: bool, total: Anyon
) -> np.ndarray:
    basis = enumerate_fusion_basis(n, total)
    dim = len(basis)
    if dim == 0:
        return np.eye(1, dtype=complex)
    R = R_matrix_tau_numeric()
    F = F_matrix_tau_numeric()
    if inverse:
        R = R.conj().T
    # For small n construct via R on adjacent pair with F-basis change
    U = np.eye(dim, dtype=complex)
    if dim == 1:
        U[0, 0] = R[0, 0] if total == Anyon.VACUUM else R[1, 1]
        return U
    # dim==2: apply R in the appropriate channel
    U = R.copy() if dim == 2 else np.eye(dim, dtype=complex)
    if dim == 2 and inverse:
        U = R.conj().T
    return U


@dataclass
class BraidWord:
    generators: List[BraidGenerator] = field(default_factory=list)
    n_strands: int = 3

    def matrix(self, total: Anyon = Anyon.VACUUM) -> np.ndarray:
        dim = fusion_space_dim(self.n_strands, total)
        U = np.eye(max(dim, 1), dtype=complex)
        for g in self.generators:
            G = braid_generator_matrix(self.n_strands, g.index, g.inverse, total)
            U = G @ U
        return U

    def append(self, index: int, inverse: bool = False) -> None:
        self.generators.append(BraidGenerator(index=index, inverse=inverse, n_strands=self.n_strands))


def estimate_approximation_error(U: np.ndarray, target: np.ndarray) -> float:
    """Frobenius distance between U and target (up to global phase)."""
    if U.shape != target.shape:
        return float("inf")
    # Remove global phase
    phase = np.vdot(target.ravel(), U.ravel())
    if abs(phase) > 1e-15:
        U = U * (np.conj(phase) / abs(phase))
    return float(np.linalg.norm(U - target, "fro"))
