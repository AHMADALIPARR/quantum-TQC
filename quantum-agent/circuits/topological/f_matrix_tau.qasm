// Fibonacci F-matrix on the ττ fusion qubit
// Encoding: one qubit = fusion channel of two Fibonacci τ anyons (basis |1>, |τ>).
// F and R are the matrices in quantum-agent/topological/fibonacci.toml.
// Global phase is dropped; u3 reproduces the braid unitary exactly up to that phase.
// P(measure 1 | input 0) = 0.61803398875
// Process fidelity to Pauli X = 0.61803398875; operator error = 0.653985660764
// Process fidelity to Hadamard = 0.985868271757; operator error = 0.119088247109
// Process fidelity to Pauli Z = 0.38196601125; operator error = 0.874032048898

// Average gate fidelity to Pauli X = 0.7453559925
// Average gate fidelity to Hadamard = 0.990578847838
// Average gate fidelity to Pauli Z = 0.587977340833
OPENQASM 2.0;
include "qelib1.inc";

qreg q[1];
creg c[1];

u3(1.809113788604763,0,-3.141592653589793) q[0];
measure q[0] -> c[0];