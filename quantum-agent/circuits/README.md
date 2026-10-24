# Circuit algorithms (OpenQASM only)

Every algorithm is a **complete, executable** OpenQASM 2.0 circuit.
No host language, no Python. Grover instances were statevector-checked
against `sin^2((2k+1) arcsin(2^{-n/2}))` with `k = floor(pi/4 * sqrt(2^n))`.
Bit order: `q[0]` is the leftmost ket bit.

| Family | Files | Ideal success |
|--------|-------|----------------|
| Plus / Bell | `plus_state.qasm`, `bell_pair.qasm` | 1/2, Bell correlation |
| Grover n=1, both marks | `grover_n1_marked{0,1}.qasm` | 1/2 (degenerate) |
| Grover n=2, all 4 marks | `grover_n2_marked{0,1,2,3}.qasm` | 1 |
| Grover n=3, all 8 marks | `grover_n3_marked{0..7}.qasm` | 121/128 |
| Grover n=4, all 16 marks | `grover_n4_marked{0..15}.qasm` | ~0.9613 (one clean ancilla on `q[4]`) |
| Fibonacci F, R, σ1, σ2 | `topological/{f_matrix_tau,r_matrix_tau,sigma1,sigma2}.qasm` | unitary, see checks |
| Braid [0,1,0] | `topological/logical_x_braid_010.qasm` | avg. fidelity 0.745 to X, 0.991 to H |
| Braid [0,1,0,1] | `topological/logical_h_braid_0101.qasm` | avg. fidelity 0.560 to H |

Null-collapse circuits: `null-collapse-tqc/circuits/`
φ⁴ Trotter: `cpsc-qft/qasm/phi4_trotter.qasm`
Numeric check record: `verification/circuit_checks.toml`
