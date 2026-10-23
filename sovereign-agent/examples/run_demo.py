#!/usr/bin/env python3
# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Minimal demo: compile, verify, execute, and print a signed receipt summary."""

from __future__ import annotations
import sys
sys.path.insert(0, "src")

from sqa.runtime import ExecutionHost, ExecutionState
from sqa.backends.qutip_sv import QuTiPStatevectorBackend
from sqa.receipts.schema import verify_receipt

PLUS = {
    "ir_version": "0.1.0",
    "qregs": [{"name": "q", "size": 1}],
    "cregs": [{"name": "c", "size": 1}],
    "operations": [
        {"gate": "H", "qubits": [{"register": "q", "index": 0}], "source_id": "h0"},
        {
            "gate": "MEASURE",
            "qubits": [{"register": "q", "index": 0}],
            "classical": {"register": "c", "index": 0},
            "source_id": "m0",
        },
    ],
    "metadata": {
        "name": "plus_state_demo",
        "objective": "Prepare |+> and measure",
    },
    "resources": {"max_qubits": 4, "max_depth": 100, "max_gates": 100},
}


def main():
    host = ExecutionHost(backend=QuTiPStatevectorBackend())
    ctx = host.run_json(PLUS)
    print("=== Execution State:", ctx.state.value)
    if ctx.result:
        print("Counts:", ctx.result.counts)
    if ctx.receipt:
        print("Program hash:", ctx.receipt.program_hash[:16] + "...")
        print("Artifact hash:", ctx.receipt.artifact_hash[:16] + "...")
        print("Signature:", ctx.receipt.signature[:16] + "...")
        ok = verify_receipt(ctx.receipt)
        print("Receipt verification:", "PASS" if ok else "FAIL")
    if ctx.error:
        print("Error:", ctx.error)
        sys.exit(1)


if __name__ == "__main__":
    main()
