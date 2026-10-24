// Fibonacci R-matrix, diag(e^{-i 4π/5}, e^{i 3π/5})
// Encoding: one qubit = fusion channel of two Fibonacci τ anyons (basis |1>, |τ>).
// F and R are the matrices in quantum-agent/topological/fibonacci.toml.
// Global phase is dropped; u3 reproduces the braid unitary exactly up to that phase.
// P(measure 1 | input 0) = 0
// Process fidelity to Pauli X = 0; operator error = 1.97537668119
// Process fidelity to Hadamard = 0.327254248594; operator error = 0.925136311235
// Process fidelity to Pauli Z = 0.654508497187; operator error = 0.61803398875

// Average gate fidelity to Pauli X = 0.333333333333
// Average gate fidelity to Hadamard = 0.551502832396
// Average gate fidelity to Pauli Z = 0.769672331458
OPENQASM 2.0;
include "qelib1.inc";

qreg q[1];
creg c[1];

u3(0,0,-1.884955592153876) q[0];
measure q[0] -> c[0];