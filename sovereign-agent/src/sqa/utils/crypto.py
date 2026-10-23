# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Lightweight signing helpers (dev HMAC; replace with real keys in production)."""

from __future__ import annotations

import hashlib
import hmac
from typing import Tuple


def hmac_sign(message: str, secret: str = "sqa-dev") -> str:
    return hmac.new(secret.encode(), message.encode(), hashlib.sha256).hexdigest()


def hmac_verify(message: str, signature: str, secret: str = "sqa-dev") -> bool:
    expected = hmac_sign(message, secret)
    return hmac.compare_digest(expected, signature)


def keypair_placeholder() -> Tuple[str, str]:
    """Dev-only placeholder public/private identifiers."""
    return ("sqa-dev-public", "sqa-dev-private")
