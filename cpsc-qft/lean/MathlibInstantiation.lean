-- File: MathlibInstantiation.lean
-- Requires: Mathlib4 (analysis.matrix.exponential, analysis.complex.basic, linear_algebra.matrix.hermitian)

import Mathlib.Analysis.Matrix.Exponential
import Mathlib.Analysis.Complex.Basic
import Mathlib.LinearAlgebra.Matrix.Hermitian
import Mathlib.LinearAlgebra.Matrix.NonsingularInverse

open Matrix Complex

noncomputable def QuantumEvolution {n : Type*} [Fintype n] (H : Matrix n n ℂ) (t : ℝ) : Matrix n n ℂ :=
  Matrix.exp (-(Complex.I * t) • H)

/-- Critical Lemma: Hermitian Hamiltonian generates Unitary Evolution. -/
theorem hermitian_exp_unitary {n : Type*} [Fintype n] (H : Matrix n n ℂ) (hH : H.Hermitian) (t : ℝ) :
    (QuantumEvolution H t).IsUnitary := by
  have h₁ : (QuantumEvolution H t)† = Matrix.exp (-(Complex.I * t) • H)† := rfl
  have h₂ : (Matrix.exp (-(Complex.I * t) • H))† = Matrix.exp ((-(Complex.I * t) • H)†) := by
    rw [Matrix.exp_adjoint]
  have h₃ : (-(Complex.I * t) • H)† = -(-(Complex.I * t) • H) := by
    calc
      (-(Complex.I * t) • H)† = (-(Complex.I * t) : ℂ)† • H† := by
        rw [Matrix.smul_adjoint, Complex.ext_iff, Complex.I_mul_I] <;> simp [Complex.ext_iff, Complex.I_mul_I] <;> norm_num <;>
        simp_all [Complex.ext_iff, Complex.I_mul_I, Matrix.adjoint_apply] <;>
        norm_num <;>
        aesop
      _ = (-(Complex.I * t) : ℂ)† • H := by rw [hH]
      _ = (-(-(Complex.I * t)) : ℂ) • H := by
        simp [Complex.ext_iff, Complex.I_mul_I, Complex.conj_re, Complex.conj_im, Complex.ext_iff, Complex.I_mul_I]
        <;> norm_num <;>
        simp_all [Complex.ext_iff, Complex.I_mul_I] <;>
        ring_nf <;> norm_num <;>
        aesop
      _ = -(-(Complex.I * t) • H) := by
        simp [smul_smul, Complex.ext_iff, Complex.I_mul_I] <;> ring_nf <;> norm_num <;>
        simp_all [Complex.ext_iff, Complex.I_mul_I] <;>
        aesop
  have h₄ : (QuantumEvolution H t)† = Matrix.exp (-(-(Complex.I * t) • H)) := by
    rw [h₁, h₂, h₃]
  have h₅ : (QuantumEvolution H t)† * QuantumEvolution H t = 1 := by
    calc
      (QuantumEvolution H t)† * QuantumEvolution H t = Matrix.exp (-(-(Complex.I * t) • H)) * Matrix.exp (-(Complex.I * t) • H) := by rw [h₄]
      _ = Matrix.exp (-(-(Complex.I * t) • H) + (-(Complex.I * t) • H)) := by
        rw [← Matrix.exp_add_comm]
        <;>
        (try simp_all [Matrix.mul_smul, Matrix.smul_mul, Complex.ext_iff, Complex.I_mul_I]) <;>
        (try ring_nf) <;>
        (try norm_num) <;>
        (try aesop)
      _ = Matrix.exp (0 : Matrix n n ℂ) := by
        congr 1
        ext i j
        simp [Matrix.add_apply, Matrix.smul_apply, Complex.ext_iff, Complex.I_mul_I]
        <;> ring_nf <;> simp_all [Complex.ext_iff, Complex.I_mul_I] <;> norm_num <;> aesop
      _ = 1 := by simp [Matrix.exp_zero]
  exact ⟨by
    have h₆ := h₅
    have h₇ : (QuantumEvolution H t)† * QuantumEvolution H t = 1 := h₅
    have h₈ : QuantumEvolution H t * (QuantumEvolution H t)† = 1 := by
      have h₉ : (QuantumEvolution H t).Nonsingular := by
        apply Matrix.nonsingular_of_mul_eq_one_right
        exact h₇
      have h₁₀ : (QuantumEvolution H t)⁻¹ = (QuantumEvolution H t)† := by
        apply Matrix.inv_eq_right_inv
        exact h₇
      calc
        QuantumEvolution H t * (QuantumEvolution H t)† = QuantumEvolution H t * (QuantumEvolution H t)⁻¹ := by rw [h₁₀]
        _ = 1 := by
          apply Matrix.mul_inv_of_nonsingular
          exact h₉
    exact h₈, h₇⟩
