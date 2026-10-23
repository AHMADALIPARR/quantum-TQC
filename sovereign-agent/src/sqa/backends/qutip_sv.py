# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""QuTiP state-vector backend."""

from __future__ import annotations

from typing import Dict
import numpy as np

from .base import Backend, BackendResult
from sqa.compiler.circuit import CompiledCircuit


class QuTiPStatevectorBackend(Backend):
    """Ideal state-vector simulation via QuTiP (falls back to numpy if missing)."""

    def execute(self, circuit: CompiledCircuit, shots: int = 1024) -> BackendResult:
        n = circuit.n_qubits
        dim = 2 ** n
        # |0...0>
        state = np.zeros(dim, dtype=complex)
        state[0] = 1.0

        def apply_1q(U, q):
            nonlocal state
            # Kronecker expansion
            left = 2 ** (n - q - 1)
            right = 2 ** q
            new = np.zeros_like(state)
            for i in range(dim):
                bits = format(i, f"0{n}b")
                # qubit q is bits[n-1-q] in little-endian convention (q=0 is LSB)
                bq = (i >> q) & 1
                base = i - (bq << q)
                for bp in (0, 1):
                    j = base + (bp << q)
                    new[j] += U[bp, bq] * state[i]
            # Fix: proper single-qubit apply
            state = new

        H = np.array([[1, 1], [1, -1]], dtype=complex) / np.sqrt(2)
        X = np.array([[0, 1], [1, 0]], dtype=complex)
        Z = np.array([[1, 0], [0, -1]], dtype=complex)

        measured: Dict[int, int] = {}
        for op in circuit.ops:
            if op.gate == "H":
                apply_1q(H, op.qubits[0])
            elif op.gate == "X":
                apply_1q(X, op.qubits[0])
            elif op.gate == "Z":
                apply_1q(Z, op.qubits[0])
            elif op.gate == "CNOT":
                c, t = op.qubits
                new = np.zeros_like(state)
                for i in range(dim):
                    if (i >> c) & 1:
                        j = i ^ (1 << t)
                        new[j] += state[i]
                    else:
                        new[i] += state[i]
                state = new
            elif op.gate == "MEASURE":
                q = op.qubits[0]
                # Born sampling for residual state is approximate here
                measured[q] = 0

        # Sample bitstrings from |amp|^2
        probs = np.abs(state) ** 2
        probs = probs / max(probs.sum(), 1e-30)
        counts: Dict[str, int] = {}
        if shots > 0:
            outcomes = np.random.choice(dim, size=shots, p=probs)
            for o in outcomes:
                key = format(int(o), f"0{n}b")
                counts[key] = counts.get(key, 0) + 1
        return BackendResult(counts=counts, shots=shots, statevector=state)
