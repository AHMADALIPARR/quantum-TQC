# Circuit algorithms (OpenQASM only)

Every algorithm is a **complete, executable** OpenQASM 2.0 circuit.
No host language, no Python.

| Circuit | Algorithm |
|---------|-----------|
| `plus_state.qasm` | Prepare \|+\>, measure |
| `bell_pair.qasm` | Bell \|Φ+\>, measure |
| `grover_n2_marked1.qasm` | Grover n=2, marked \|01\> |
| `grover_n2_marked3.qasm` | Grover n=2, marked \|11\> |

Null-collapse circuits: `null-collapse-tqc/circuits/`
φ⁴ Trotter: `cpsc-qft/qasm/phi4_trotter.qasm`
