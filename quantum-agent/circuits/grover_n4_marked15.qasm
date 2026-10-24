// Algorithm: Grover search
// n = 4 qubits, N = 16, marked state |1111> (integer 15)
// Bit order: q[0] is the leftmost ket bit (MSB of the marked integer).
// Iterations = floor(pi/4 * sqrt(2^4)) = 3
// Ideal success probability = sin^2((2k+1) arcsin(1/sqrt(N))) = 0.961318969727
// q[4] is a clean workspace ancilla for the 3-controlled phase. It returns to |0> and is not measured.
// Oracle: X on bits that are 0 in the marked string, multi-controlled Z, uncompute.
// Diffusion: H, X, multi-controlled Z, X, H on the search register.

OPENQASM 2.0;
include "qelib1.inc";

qreg q[5];
creg c[4];

// ---- Uniform superposition ----
h q[0];
h q[1];
h q[2];
h q[3];

// ---- Iteration 1: oracle for |1111> ----
// marked string is |1...1>; no bit flips
h q[3];
ccx q[0], q[1], q[4];
ccx q[4], q[2], q[3];
ccx q[0], q[1], q[4];
h q[3];
// ---- Iteration 1: diffusion ----
h q[0];
h q[1];
h q[2];
h q[3];
x q[0];
x q[1];
x q[2];
x q[3];
h q[3];
ccx q[0], q[1], q[4];
ccx q[4], q[2], q[3];
ccx q[0], q[1], q[4];
h q[3];
x q[0];
x q[1];
x q[2];
x q[3];
h q[0];
h q[1];
h q[2];
h q[3];

// ---- Iteration 2: oracle for |1111> ----
// marked string is |1...1>; no bit flips
h q[3];
ccx q[0], q[1], q[4];
ccx q[4], q[2], q[3];
ccx q[0], q[1], q[4];
h q[3];
// ---- Iteration 2: diffusion ----
h q[0];
h q[1];
h q[2];
h q[3];
x q[0];
x q[1];
x q[2];
x q[3];
h q[3];
ccx q[0], q[1], q[4];
ccx q[4], q[2], q[3];
ccx q[0], q[1], q[4];
h q[3];
x q[0];
x q[1];
x q[2];
x q[3];
h q[0];
h q[1];
h q[2];
h q[3];

// ---- Iteration 3: oracle for |1111> ----
// marked string is |1...1>; no bit flips
h q[3];
ccx q[0], q[1], q[4];
ccx q[4], q[2], q[3];
ccx q[0], q[1], q[4];
h q[3];
// ---- Iteration 3: diffusion ----
h q[0];
h q[1];
h q[2];
h q[3];
x q[0];
x q[1];
x q[2];
x q[3];
h q[3];
ccx q[0], q[1], q[4];
ccx q[4], q[2], q[3];
ccx q[0], q[1], q[4];
h q[3];
x q[0];
x q[1];
x q[2];
x q[3];
h q[0];
h q[1];
h q[2];
h q[3];

// ---- Measure search register ----
measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];
measure q[3] -> c[3];
