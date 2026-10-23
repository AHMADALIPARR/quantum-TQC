# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Fibonacci anyon braid compiler — logical gates from braid words."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import List, Optional, Tuple
import hashlib
import json
import numpy as np

from .fibonacci import Anyon, verify_pentagon_hexagon, PHI_F
from .braid import (
    BraidWord,
    BraidGenerator,
    braid_generator_matrix,
    fusion_space_dim,
    enumerate_fusion_basis,
    estimate_approximation_error,
)


def _logical_basis_order(n: int = 3, total: Anyon = Anyon.TAU) -> list:
    return enumerate_fusion_basis(n, total)


def _matrix_to_list(U: np.ndarray) -> list:
    return [[complex(z).real if abs(complex(z).imag) < 1e-15 else complex(z)
             for z in row] for row in U.tolist()]


@dataclass
class BraidArtifact:
    n_strands: int
    word: List[dict]
    matrix: list
    total_charge: str
    content_hash: str
    approximation_error: Optional[float] = None
    target_name: Optional[str] = None


class FibonacciBraidCompiler:
    """Compile logical operations to Fibonacci braid words (n≤4)."""

    def __init__(self, n_strands: int = 3):
        if n_strands > 4:
            raise NotImplementedError("n>4 not supported")
        self.n_strands = n_strands

    def compile_logical_x(self) -> BraidArtifact:
        """Logical X on 3-anyon qubit (total τ)."""
        word = BraidWord(n_strands=self.n_strands)
        # σ_1 σ_2 σ_1  (standard Ising/Fibonacci logical X braid)
        for i in [0, 1, 0]:
            word.append(i)
        U = word.matrix(Anyon.TAU)
        payload = {
            "n": self.n_strands,
            "word": [{"i": g.index, "inv": g.inverse} for g in word.generators],
        }
        h = hashlib.sha256(json.dumps(payload, sort_keys=True).encode()).hexdigest()
        return BraidArtifact(
            n_strands=self.n_strands,
            word=payload["word"],
            matrix=_matrix_to_list(U),
            total_charge="tau",
            content_hash=h,
            target_name="logical_X",
        )

    def compile_logical_h_approx(self) -> BraidArtifact:
        """Approximate logical H; error is reported."""
        word = BraidWord(n_strands=self.n_strands)
        for i in [0, 1, 0, 1]:
            word.append(i)
        U = word.matrix(Anyon.TAU)
        target = np.array([[1, 1], [1, -1]], dtype=complex) / np.sqrt(2)
        if U.shape == target.shape:
            err = estimate_approximation_error(U, target)
        else:
            err = float("nan")
        payload = {
            "n": self.n_strands,
            "word": [{"i": g.index, "inv": g.inverse} for g in word.generators],
        }
        h = hashlib.sha256(json.dumps(payload, sort_keys=True).encode()).hexdigest()
        return BraidArtifact(
            n_strands=self.n_strands,
            word=payload["word"],
            matrix=_matrix_to_list(U),
            total_charge="tau",
            content_hash=h,
            approximation_error=err,
            target_name="logical_H_approx",
        )


def model_consistency_check() -> tuple[bool, list[str]]:
    return verify_pentagon_hexagon()
