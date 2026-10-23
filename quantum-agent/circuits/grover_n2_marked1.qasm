// Algorithm: Grover search
// n = 2 qubits, N = 4, marked state |01> (integer 1)
// Optimal iterations = floor(π/4 * √4) = 1
// Fully expanded circuit — oracle + diffusion explicit

OPENQASM 2.0;
include "qelib1.inc";

qreg q[2];
creg c[2];

// ---- Uniform superposition ----
h q[0];
h q[1];

// ---- Oracle for marked |01> ----
// Map |01> → |11>, apply CZ, unmap
x q[0];
h q[1];
cx q[0], q[1];
h q[1];
x q[0];

// ---- Diffusion (2*|+><+| - I) ----
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

// ---- Measure ----
measure q[0] -> c[0];
measure q[1] -> c[1];
