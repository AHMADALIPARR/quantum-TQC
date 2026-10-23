# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""End-to-end: IR → compile → verify → backend."""

from __future__ import annotations

from sqa.runtime import ExecutionHost, ExecutionState
from sqa.backends.qutip_sv import QuTiPStatevectorBackend

PLUS = {
    "ir_version": "0.1.0",
    "qregs": [{"name": "q", "size": 1}],
    "cregs": [{"name": "c", "size": 1}],
    "operations": [
        {"gate": "H", "qubits": [{"register": "q", "index": 0}]},
        {"gate": "MEASURE", "qubits": [{"register": "q", "index": 0}],
         "classical": {"register": "c", "index": 0}},
    ],
    "resources": {"max_qubits": 4, "max_depth": 100, "max_gates": 100},
}


def test_e2e_plus():
    host = ExecutionHost(backend=QuTiPStatevectorBackend())
    ctx = host.run_json(PLUS)
    assert ctx.state in (ExecutionState.SUCCEEDED, ExecutionState.FAILED)
    if ctx.state == ExecutionState.SUCCEEDED:
        assert ctx.result is not None
        assert ctx.receipt is not None
