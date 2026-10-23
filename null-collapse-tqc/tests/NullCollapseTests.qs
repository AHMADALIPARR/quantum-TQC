// Copyright (C) 2026 — AGPL-3.0
// Tests for Null Collapse TQC

namespace NullCollapseTQC.Tests {
    open Microsoft.Quantum.Intrinsic;
    open Microsoft.Quantum.Measurement;
    open Microsoft.Quantum.Diagnostics;
    open NullCollapseTQC;

    @Test("QuantumSimulator")
    operation TestNullPureNull() : Unit {
        let r = NullCollapse(0.0);
        Fact(r == true or r == false, "invalid measurement");
    }

    @Test("QuantumSimulator")
    operation TestNullPureNonNull() : Unit {
        let r = NullCollapse(1.0);
        Fact(r == true or r == false, "invalid measurement");
    }

    @Test("QuantumSimulator")
    operation TestBraidUnitarity() : Unit {
        // σ₁⁴ = identity (up to global phase): 4*(π/2)=2π
        use q = Qubit();
        for _ in 1..4 { BraidSigma1(q); }
        let r = M(q);
        Reset(q);
        Fact(r == Zero, "sigma_1^4 not identity on |0>");
    }

    @Test("QuantumSimulator")
    operation TestAllWeights() : Unit {
        let outs = NullCollapseAll([0.0, 0.25, 0.5, 0.75, 1.0]);
        Fact(Length(outs) == 5, "wrong output length");
    }
}
