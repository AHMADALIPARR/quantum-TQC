// Generator σ2 = F R F (F = F^{-1})
// Encoding: one qubit = fusion channel of two Fibonacci τ anyons (basis |1>, |τ>).
// F and R are the matrices in quantum-agent/topological/fibonacci.toml.
// Global phase is dropped; u3 reproduces the braid unitary exactly up to that phase.
// P(measure 1 | input 0) = 0.61803398875
// Process fidelity to Pauli X = 0.61803398875; operator error = 0.653985660764
// Process fidelity to Hadamard = 0.177112695593; operator error = 1.0762457044
// Process fidelity to Pauli Z = 0.0364745084376; operator error = 1.27201964951

// Average gate fidelity to Pauli X = 0.7453559925
// Average gate fidelity to Hadamard = 0.451408463729
// Average gate fidelity to Pauli Z = 0.357649672292
OPENQASM 2.0;
include "qelib1.inc";

qreg q[1];
creg c[1];

u3(1.809113788604763,-4.39822971502571,-7.539822368615503) q[0];
measure q[0] -> c[0];