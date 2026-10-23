# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Execution host — state machine, verification gate, backend dispatch."""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Optional, Dict
import time

from sqa.ir.schema import QuantumProgram
from sqa.ir.parser import parse_and_typecheck, program_hash
from sqa.compiler.circuit import compile_program, CompiledCircuit
from sqa.verification.engine import VerificationEngine
from sqa.backends.base import Backend, BackendResult
from sqa.receipts.schema import ExecutionReceipt, sign_receipt


class ExecutionState(str, Enum):
    CREATED = "CREATED"
    PARSED = "PARSED"
    TYPECHECKED = "TYPECHECKED"
    COMPILED = "COMPILED"
    VERIFIED = "VERIFIED"
    RUNNING = "RUNNING"
    SUCCEEDED = "SUCCEEDED"
    FAILED = "FAILED"


@dataclass
class RuntimeContext:
    program: Optional[QuantumProgram] = None
    compiled: Optional[CompiledCircuit] = None
    state: ExecutionState = ExecutionState.CREATED
    program_hash: Optional[str] = None
    result: Optional[BackendResult] = None
    receipt: Optional[ExecutionReceipt] = None
    error: Optional[str] = None
    timestamps: Dict[str, float] = field(default_factory=dict)


class ExecutionHost:
    """Host that enforces: parse → typecheck → compile → verify → run → receipt."""

    def __init__(self, backend: Backend, verifier: Optional[VerificationEngine] = None):
        self.backend = backend
        self.verifier = verifier or VerificationEngine()

    def run_json(self, ir_json: dict | str) -> RuntimeContext:
        ctx = RuntimeContext()
        ctx.timestamps["start"] = time.time()
        try:
            program = parse_and_typecheck(ir_json)
            ctx.program = program
            ctx.program_hash = program_hash(program)
            ctx.state = ExecutionState.TYPECHECKED

            compiled = compile_program(program)
            ctx.compiled = compiled
            ctx.state = ExecutionState.COMPILED

            ok, msgs = self.verifier.verify(program, compiled)
            if not ok:
                ctx.state = ExecutionState.FAILED
                ctx.error = "; ".join(msgs)
                return ctx
            ctx.state = ExecutionState.VERIFIED

            ctx.state = ExecutionState.RUNNING
            result = self.backend.execute(compiled)
            ctx.result = result
            ctx.state = ExecutionState.SUCCEEDED

            ctx.receipt = sign_receipt(
                program_hash=ctx.program_hash,
                artifact_hash=compiled.artifact_hash(),
                result_summary=str(result.counts) if result else "",
            )
        except Exception as e:
            ctx.state = ExecutionState.FAILED
            ctx.error = str(e)
        ctx.timestamps["end"] = time.time()
        return ctx
