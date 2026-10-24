// Algorithm: Grover search
// n = 2 qubits, N = 4, marked state |10> (integer 2)
// Bit order: q[0] is the leftmost ket bit (MSB of the marked integer).
// Iterations = floor(pi/4 * sqrt(2^2)) = 1
// Ideal success probability = sin^2((2k+1) arcsin(1/sqrt(N))) = 1 = 1/1
// Oracle: X on bits that are 0 in the marked string, multi-controlled Z, uncompute.
// Diffusion: H, X, multi-controlled Z, X, H on the search register.

OPENQASM 2.0;
include "qelib1.inc";

qreg q[2];
creg c[2];

// ---- Uniform superposition ----
h q[0];
h q[1];

// ---- Iteration 1: oracle for |10> ----
x q[1];
h q[1];
cx q[0], q[1];
h q[1];
x q[1];
// ---- Iteration 1: diffusion ----
h q[0];
h q[1];
x q[0];
x q[1];
h q[1];
cx q[0], q[1];
h q[1];
x q[0];
x q[1];
h q[0];
h q[1];

// ---- Measure search register ----
measure q[0] -> c[0];
measure q[1] -> c[1];
