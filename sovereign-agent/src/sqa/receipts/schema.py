# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Signed execution receipts."""

from __future__ import annotations

from dataclasses import dataclass, asdict
from typing import Optional
import hashlib
import json
import time


@dataclass
class ExecutionReceipt:
    program_hash: str
    artifact_hash: str
    result_summary: str
    timestamp: float
    signature: str

    def to_dict(self) -> dict:
        return asdict(self)


def sign_receipt(program_hash: str, artifact_hash: str, result_summary: str, secret: str = "sqa-dev") -> ExecutionReceipt:
    ts = time.time()
    payload = f"{program_hash}|{artifact_hash}|{result_summary}|{ts}|{secret}"
    sig = hashlib.sha256(payload.encode()).hexdigest()
    return ExecutionReceipt(
        program_hash=program_hash,
        artifact_hash=artifact_hash,
        result_summary=result_summary,
        timestamp=ts,
        signature=sig,
    )


def verify_receipt(receipt: ExecutionReceipt, secret: str = "sqa-dev") -> bool:
    payload = f"{receipt.program_hash}|{receipt.artifact_hash}|{receipt.result_summary}|{receipt.timestamp}|{secret}"
    expected = hashlib.sha256(payload.encode()).hexdigest()
    return expected == receipt.signature
