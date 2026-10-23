# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Typed Quantum Intermediate Representation (IR) package."""

from .schema import (
    IR_VERSION,
    SUPPORTED_GATES,
    GateType,
    QubitRef,
    ClassicalBitRef,
    GateOp,
    QuantumRegister,
    ClassicalRegister,
    ResourceConstraints,
    AlgorithmMetadata,
    BackendRequirements,
    QuantumProgram,
    IRError,
    SchemaError,
    TypeCheckError,
    ResourceError,
)
from .typecheck import type_check_program, is_well_typed
from .parser import (
    parse_program,
    parse_and_typecheck,
    serialize_program,
    program_hash,
    load_program_file,
    save_program_file,
)

__all__ = [
    "IR_VERSION", "SUPPORTED_GATES", "GateType", "QubitRef", "ClassicalBitRef",
    "GateOp", "QuantumRegister", "ClassicalRegister", "ResourceConstraints",
    "AlgorithmMetadata", "BackendRequirements", "QuantumProgram",
    "IRError", "SchemaError", "TypeCheckError", "ResourceError",
    "type_check_program", "is_well_typed", "parse_program", "parse_and_typecheck",
    "serialize_program", "program_hash", "load_program_file", "save_program_file",
]
