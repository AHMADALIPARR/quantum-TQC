# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Content-addressed hashing helpers."""

from __future__ import annotations

import hashlib
import json
from typing import Any


def canonical_json(obj: Any) -> str:
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def sha256_hex(data: bytes | str) -> str:
    if isinstance(data, str):
        data = data.encode("utf-8")
    return hashlib.sha256(data).hexdigest()


def program_hash(program: dict) -> str:
    return sha256_hex(canonical_json(program))
