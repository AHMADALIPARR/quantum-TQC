# Null Collapse Algorithm (Ising TQC / Quantum Euclidean)

## Operator Objective

Collapse a search space to the null (vacuum) sector using TQC over the Euclidean plane
with a Quantum Euclidean structure on configuration space. Search space is encoded in
fusion outcomes of **Ising anyons**; braid words + topological charge measurement
collapse candidates to the vacuum sector.

## Primitives

- Ambient space: R², configuration space C_n(R²).
- Quantum Euclidean Hilbert space H_n of L² sections over fusion trees.
- Ising anyons {1, σ, ψ}:
  - 1×x=x, σ×σ=1+ψ, σ×ψ=σ, ψ×ψ=1.
- Braid group B_n acts unitarily: ρ: B_n → U(H_n).
- Null sector: all adjacent pairs fuse to 1 (vacuum) — |0_null⟩.

## 3-anyon instance

Intermediate fusion charge of first two σ-anyons is a qubit:

- |0⟩ ≡ 1×σ (null / annihilation channel)
- |1⟩ ≡ ψ×σ (non-null channel)

Braid generators (up to global phase e^{-iπ/8}):

- σ₁ = R₁(π/2) = diag(1, i)
- σ₂ = H · σ₁ · H

Search braid: β = σ₁ σ₂ σ₁.

## Algorithm

```
Input:  candidate weight w ∈ [0,1]
Output: Boolean null (True iff collapsed to vacuum)

1. theta ← 2 · arcsin(w)
2. prepare q ← Ry(theta)|0⟩
3. apply SearchBraid(q) = σ₁ · σ₂ · σ₁
4. r ← measure q
5. reset q
6. return (r == 0)
```

Multi-candidate: independent pairs separated in the plane (braids commute); all 0 ⇒ full null collapse.

## Proof obligations

- **P1** Unitarity of each σᵢ and any braid word.
- **P2** Topological invariance (homotopy of path in C_n).
- **P3** Measurement projects onto charge eigenspaces; 0 → null sector.
- **P4** Correctness of null detection for w=0 and general w.
- **P5** Euclidean well-definedness of H_n inner product.

## Novelty

EQUIVALENT_TO_KNOWN_METHOD — standard Ising TQC (Freedman–Kitaev–Wang; Bravyi–Kitaev);
"null collapse" is an algorithmic reinterpretation of topological charge measurement.
No new complexity separation claimed.
