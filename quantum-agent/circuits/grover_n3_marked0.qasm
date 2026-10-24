// Algorithm: Grover search
// n = 3 qubits, N = 8, marked state |000> (integer 0)
// Bit order: q[0] is the leftmost ket bit (MSB of the marked integer).
// Iterations = floor(pi/4 * sqrt(2^3)) = 2
// Ideal success probability = sin^2((2k+1) arcsin(1/sqrt(N))) = 0.9453125 = 121/128
// Oracle: X on bits that are 0 in the marked string, multi-controlled Z, uncompute.
// Diffusion: H, X, multi-controlled Z, X, H on the search register.

OPENQASM 2.0;
include "qelib1.inc";

qreg q[3];
creg c[3];

// ---- Uniform superposition ----
h q[0];
h q[1];
h q[2];

// ---- Iteration 1: oracle for |000> ----
x q[0];
x q[1];
x q[2];
h q[2];
ccx q[0], q[1], q[2];
h q[2];
x q[0];
x q[1];
x q[2];
// ---- Iteration 1: diffusion ----
h q[0];
h q[1];
h q[2];
x q[0];
x q[1];
x q[2];
h q[2];
ccx q[0], q[1], q[2];
h q[2];
x q[0];
x q[1];
x q[2];
h q[0];
h q[1];
h q[2];

// ---- Iteration 2: oracle for |000> ----
x q[0];
x q[1];
x q[2];
h q[2];
ccx q[0], q[1], q[2];
h q[2];
x q[0];
x q[1];
x q[2];
// ---- Iteration 2: diffusion ----
h q[0];
h q[1];
h q[2];
x q[0];
x q[1];
x q[2];
h q[2];
ccx q[0], q[1], q[2];
h q[2];
x q[0];
x q[1];
x q[2];
h q[0];
h q[1];
h q[2];

// ---- Measure search register ----
measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];
