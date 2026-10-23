# Copyright (C) 2026 NOVA PARR / Sovereign Quantum Agent Framework
# Licensed under GNU AGPL v3.0 — see LICENSE

"""Oracle interface and linear Boolean oracles."""

from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import List, Tuple

from sqa.ir.schema import GateOp, QubitRef


@dataclass
class OracleContract:
    input_bits: int
    output_bits: int
    reversible: bool = True


class BooleanOracle(ABC):
    @abstractmethod
    def classical_eval(self, bits: List[int]) -> List[int]:
        ...

    @abstractmethod
    def synthesize(self, input_reg: str, output_reg: str) -> List[GateOp]:
        ...

    @abstractmethod
    def contract(self) -> OracleContract:
        ...


class LinearBooleanOracle(BooleanOracle):
    """f(x) = parity of bits selected by mask."""

    def __init__(self, name: str, n: int, mask: int):
        self.name = name
        self.n = n
        self.mask = mask

    def classical_eval(self, bits: List[int]) -> List[int]:
        assert len(bits) == self.n
        acc = 0
        for i, b in enumerate(bits):
            if (self.mask >> i) & 1:
                acc ^= b
        return [acc]

    def synthesize(self, input_reg: str, output_reg: str) -> List[GateOp]:
        ops = []
        for i in range(self.n):
            if (self.mask >> i) & 1:
                ops.append(GateOp(
                    gate="CNOT",
                    qubits=[
                        QubitRef(register=input_reg, index=i),
                        QubitRef(register=output_reg, index=0),
                    ],
                ))
        return ops

    def contract(self) -> OracleContract:
        return OracleContract(input_bits=self.n, output_bits=1, reversible=True)

    def verify_classical_truth_table(self) -> Tuple[bool, List[str]]:
        msgs = []
        for x in range(2 ** self.n):
            bits = [(x >> i) & 1 for i in range(self.n)]
            y = self.classical_eval(bits)[0]
            expected = 0
            for i in range(self.n):
                if (self.mask >> i) & 1:
                    expected ^= bits[i]
            if y != expected:
                return False, [f"mismatch at {bits}"]
        msgs.append("truth table ok")
        return True, msgs
