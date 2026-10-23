namespace NullCollapseTQC {
    open Microsoft.Quantum.Intrinsic;
    open Microsoft.Quantum.Math;
    open Microsoft.Quantum.Measurement;
    open Microsoft.Quantum.Arrays;
    open Microsoft.Quantum.Canon;

    // Single-qubit braid generators (Ising, 3-sigma fusion space)
    operation BraidSigma1(q : Qubit) : Unit is Adj + Ctl {
        R1(PI() / 2.0, q);
    }

    operation BraidSigma1Squared(q : Qubit) : Unit is Adj + Ctl {
        R1(PI(), q); // sigma_1^2 = R1(pi) = Z
    }

    operation BraidSigma2(q : Qubit) : Unit is Adj + Ctl {
        H(q);
        R1(PI() / 2.0, q);
        H(q);
    }

    // Two-qubit braid generator sigma_{2k} (between pair k and k+1)
    operation BraidSigmaEven(control : Qubit, target : Qubit) : Unit is Adj + Ctl {
        CNOT(control, target);
        H(control);
        H(target);
        CNOT(control, target);
    }

    // Local concentration step: Z on each qubit (sigma_{2k-1}^2)
    operation LocalConcentration(qs : Qubit[]) : Unit is Adj + Ctl {
        ApplyToEachA(BraidSigma1Squared, qs);
    }

    // Pairwise entangling reflection step
    operation PairwiseReflection(qs : Qubit[]) : Unit is Adj + Ctl {
        for i in 0..Length(qs)-2 {
            BraidSigmaEven(qs[i], qs[i+1]);
        }
    }

    // Full concentration round (Clifford amplitude amplification step)
    operation ConcentrationRound(qs : Qubit[]) : Unit is Adj + Ctl {
        LocalConcentration(qs);
        PairwiseReflection(qs);
    }

    // Multi-candidate null collapse with optional concentration rounds
    operation NullCollapseMulti(weights : Double[], concentrationRounds : Int) : Bool[] {
        let m = Length(weights);
        use qs = Qubit[m];
        
        // Encode weights
        for i in 0..m-1 {
            let theta = 2.0 * ArcSin(weights[i]);
            Ry(theta, qs[i]);
        }

        // Apply concentration rounds
        for _ in 1..concentrationRounds {
            ConcentrationRound(qs);
        }

        // Measure all qubits
        mutable outcomes = [];
        for i in 0..m-1 {
            let r = M(qs[i]);
            set outcomes += [r == Zero];
            Reset(qs[i]);
        }
        return outcomes;
    }

    // Convenience: single round (local Z only)
    operation NullCollapseLocal(weights : Double[]) : Bool[] {
        return NullCollapseMulti(weights, 1);
    }

    // Convenience: no concentration (baseline)
    operation NullCollapseBaseline(weights : Double[]) : Bool[] {
        return NullCollapseMulti(weights, 0);
    }
}
