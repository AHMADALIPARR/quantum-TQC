// Algorithm: Grover search
// n = 1 qubits, N = 2, marked state |1> (integer 1)
// Bit order: q[0] is the leftmost ket bit (MSB of the marked integer).
// Iterations = floor(pi/4 * sqrt(2^1)) = 1
// Ideal success probability = sin^2((2k+1) arcsin(1/sqrt(N))) = 0.5 = 1/2
// n = 1 is degenerate: one standard Grover iterate does not beat a fair coin.
// Oracle: X on bits that are 0 in the marked string, multi-controlled Z, uncompute.
// Diffusion: H, X, multi-controlled Z, X, H on the search register.

OPENQASM 2.0;
include "qelib1.inc";

qreg q[1];
creg c[1];

// ---- Uniform superposition ----
h q[0];

// ---- Iteration 1: oracle for |1> ----
// marked string is |1...1>; no bit flips
z q[0];
// ---- Iteration 1: diffusion ----
h q[0];
x q[0];
z q[0];
x q[0];
h q[0];

// ---- Measure search register ----
measure q[0] -> c[0];
