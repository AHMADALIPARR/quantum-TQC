// Copyright (C) 2026
// Licensed under the GNU Affero General Public License v3.0.
// Null Collapse TQC — Ising anyon search braid + charge measurement

namespace NullCollapseTQC {
    open Microsoft.Quantum.Intrinsic;
    open Microsoft.Quantum.Math;
    open Microsoft.Quantum.Measurement;

    /// Ising braid generator σ₁ = R1(π/2) on the 2-dim fusion space of 3 σ-anyons.
    /// |0⟩ = intermediate charge 1 (null / annihilation)
    /// |1⟩ = intermediate charge ψ (non-null)
    operation BraidSigma1(q : Qubit) : Unit is Adj + Ctl {
        R1(PI() / 2.0, q);
    }

    /// σ₂ = H · σ₁ · H  (F-matrix conjugation)
    operation BraidSigma2(q : Qubit) : Unit is Adj + Ctl {
        H(q);
        R1(PI() / 2.0, q);
        H(q);
    }

    /// Search braid β = σ₁ σ₂ σ₁
    operation SearchBraid(q : Qubit) : Unit is Adj + Ctl {
        BraidSigma1(q);
        BraidSigma2(q);
        BraidSigma1(q);
    }

    /// Single-candidate null collapse.
    /// Returns true iff measurement projects to null sector (Zero).
    operation NullCollapse(candidateWeight : Double) : Bool {
        use q = Qubit();
        let theta = 2.0 * ArcSin(candidateWeight);
        Ry(theta, q);
        SearchBraid(q);
        let r = M(q);
        Reset(q);
        return r == Zero;
    }

    /// Multi-candidate: independent pairs (Euclidean-separated in the plane).
    operation NullCollapseAll(weights : Double[]) : Bool[] {
        mutable outcomes = [];
        for w in weights {
            set outcomes += [NullCollapse(w)];
        }
        return outcomes;
    }
}
