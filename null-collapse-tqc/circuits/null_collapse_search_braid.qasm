// Null Collapse Algorithm — full executable circuit
// Ising TQC: encode w → SearchBraid(σ1 σ2 σ1) → measure fusion charge
// |0> = null (vacuum) sector; |1> = non-null
// Example: w = 0.5 → theta = 2*arcsin(0.5) = π/3

OPENQASM 2.0;
include "qelib1.inc";

qreg q[1];
creg c[1];

// 1. Encode candidate weight w=0.5
ry(1.0471975511965976) q[0];

// 2. SearchBraid = σ1 · σ2 · σ1
//    σ1 = R1(π/2) ≡ u1(π/2)
//    σ2 = H · σ1 · H

// --- σ1 ---
u1(1.5707963267948966) q[0];

// --- σ2 = H σ1 H ---
h q[0];
u1(1.5707963267948966) q[0];
h q[0];

// --- σ1 ---
u1(1.5707963267948966) q[0];

// 3. Measure intermediate fusion charge
//    c==0 ⇒ collapsed to null sector
measure q[0] -> c[0];
