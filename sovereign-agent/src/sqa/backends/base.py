# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Backend interface."""

from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from typing import Dict, Optional, Any

from sqa.compiler.circuit import CompiledCircuit


@dataclass
class BackendResult:
    counts: Dict[str, int] = field(default_factory=dict)
    shots: int = 0
    statevector: Optional[Any] = None
    metadata: Dict[str, Any] = field(default_factory=dict)


class Backend(ABC):
    @abstractmethod
    def execute(self, circuit: CompiledCircuit, shots: int = 1024) -> BackendResult:
        ...
