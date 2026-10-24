// Framework logical H braid word [0, 1, 0, 1] = σ1 σ2 σ1 σ2
// Encoding: one qubit = fusion channel of two Fibonacci τ anyons (basis |1>, |τ>).
// F and R are the matrices in quantum-agent/topological/fibonacci.toml.
// Global phase is dropped; u3 reproduces the braid unitary exactly up to that phase.
// P(measure 1 | input 0) = 0.61803398875
// Process fidelity to Pauli X = 0.213525491562; operator error = 1.03721918039
// Process fidelity to Hadamard = 0.340609110784; operator error = 0.91255984576
// Process fidelity to Pauli Z = 0.13196601125; operator error = 1.12847572947
// Average gate fidelity to Pauli X = 0.475683661041
// Average gate fidelity to Hadamard = 0.560406073856
// Average gate fidelity to Pauli Z = 0.421310674167
// Framework marks this word as approximate logical H. Under the supplied F and R it is not close to H.
// The circuit is the braid the framework names, not a relabeled Hadamard.

OPENQASM 2.0;
include "qelib1.inc";

qreg q[1];
creg c[1];

u3(1.809113788604763,0,1.256637061435917) q[0];
measure q[0] -> c[0];