// Framework logical X braid word [0, 1, 0] = σ1 σ2 σ1
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
// Framework marks this word exact logical X. Under the supplied F and R it is not X.
// It is the standard approximate Hadamard weave (fidelity to H above). Error is reported, not hidden.

OPENQASM 2.0;
include "qelib1.inc";

qreg q[1];
creg c[1];

u3(1.809113788604763,0,3.141592653589793) q[0];
measure q[0] -> c[0];