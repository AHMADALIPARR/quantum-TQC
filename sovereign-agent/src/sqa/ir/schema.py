# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Typed Quantum IR schema."""

from __future__ import annotations

from dataclasses import dataclass, field, asdict
from enum import Enum
from typing import Any, Optional, List

IR_VERSION = "0.1.0"
SUPPORTED_GATES = ("H", "X", "Z", "CNOT", "MEASURE")


class GateType(str, Enum):
    H = "H"
    X = "X"
    Z = "Z"
    CNOT = "CNOT"
    MEASURE = "MEASURE"


class IRError(Exception):
    pass


class SchemaError(IRError):
    pass


class TypeCheckError(IRError):
    pass


class ResourceError(IRError):
    pass


@dataclass
class QubitRef:
    register: str
    index: int


@dataclass
class ClassicalBitRef:
    register: str
    index: int


@dataclass
class GateOp:
    gate: str
    qubits: List[QubitRef]
    classical: Optional[ClassicalBitRef] = None
    source_id: Optional[str] = None


@dataclass
class QuantumRegister:
    name: str
    size: int


@dataclass
class ClassicalRegister:
    name: str
    size: int


@dataclass
class ResourceConstraints:
    max_qubits: int = 8
    max_depth: int = 10_000
    max_gates: int = 50_000
    numerical_tolerance: float = 1e-10


@dataclass
class AlgorithmMetadata:
    name: str = ""
    objective: str = ""
    description: str = ""
    assumptions: List[str] = field(default_factory=list)


@dataclass
class BackendRequirements:
    min_qubits: int = 1
    supports_statevector: bool = True
    supports_measurement: bool = True
    native_gates: List[str] = field(default_factory=lambda: list(SUPPORTED_GATES))


@dataclass
class QuantumProgram:
    ir_version: str
    qregs: List[QuantumRegister]
    cregs: List[ClassicalRegister]
    operations: List[GateOp]
    metadata: Optional[AlgorithmMetadata] = None
    resources: Optional[ResourceConstraints] = None
    backend_requirements: Optional[BackendRequirements] = None

    def to_dict(self) -> dict:
        return asdict(self)
