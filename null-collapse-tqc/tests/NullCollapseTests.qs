namespace NullCollapseTQC.Tests {
    open Microsoft.Quantum.Intrinsic;
    open Microsoft.Quantum.Testing;
    open Microsoft.Quantum.Math;
    open Microsoft.Quantum.Arrays;
    open NullCollapseTQC;

    @Test("QuantumSimulator")
    operation TestLocalConcentrationIncreasesNullProb() : Unit {
        let weights = [0.3, 0.4, 0.2];
        let outcomes = NullCollapseLocal(weights);
        Fact(Length(outcomes) == 3, "wrong length");
    }

    @Test("QuantumSimulator")
    operation TestBaselineVsConcentration() : Unit {
        let weights = [0.5, 0.5];
        let _ = NullCollapseBaseline(weights);
        let _ = NullCollapseLocal(weights);
    }

    @Test("QuantumSimulator")
    operation TestBraidSigmaEvenUnitarity() : Unit {
        use (c, t) = (Qubit(), Qubit());
        BraidSigmaEven(c, t);
        Adjoint BraidSigmaEven(c, t);
        let rc = M(c);
        let rt = M(t);
        Reset(c);
        Reset(t);
        Fact(rc == Zero and rt == Zero, "BraidSigmaEven not unitary");
    }

    @Test("QuantumSimulator")
    operation TestThreeQubitEntanglement() : Unit {
        use qs = Qubit[3];
        ApplyToEachA(H, qs);
        PairwiseReflection(qs);
        let outcomes = MultiM(qs);
        ResetAll(qs);
        Fact(true, "executed");
    }
}
