# Capability Matrix

| Feature | Implemented | Tested | Simulated | Formally Verified | Hardware-Validated |
|---------|-------------|--------|-----------|-------------------|--------------------|
| **Milestone 1** | | | | | |
| Typed IR v0.1.0 (H,X,CNOT,MEASURE) | Yes | Yes | — | No | No |
| Independent verification engine | Yes | Yes | — | No | No |
| QuTiP state-vector backend | Yes | Yes | Yes | No | No |
| Signed receipts + tamper detection | Yes | Yes | — | No | No |
| **Milestone 2** | | | | | |
| Typed oracle interface + contracts | Yes | Yes | — | No | No |
| Linear Boolean oracle | Yes | Yes | — | No | No |
| Optimization passes + contracts | Yes | Yes | — | No | No |
| Small Grover (n≤4) | Yes | Yes | Yes | No | No |
| **Milestone 3** | | | | | |
| Fibonacci fusion rules | Yes | Yes | — | No | No |
| F-symbols (exact SymPy + numeric) | Yes | Yes | — | No | No |
| R-symbols (exact SymPy + numeric) | Yes | Yes | — | No | No |
| Pentagon / hexagon consistency checks | Yes | Yes (numerical) | — | No | No |
| Fusion-space dimension & basis | Yes | Yes | — | No | No |
| Braid generators σ_i on n≤4 | Yes | Yes | — | No | No |
| Braid-word unitary compilation | Yes | Yes | — | No | No |
| Logical X braid (3-anyon qubit) | Yes | Yes | — | No | No |
| Approximate logical H braid | Yes | Yes | — | No | No |
| Approximation-error reporting | Yes | Yes | — | No | No |
| Braid artifact + content hash | Yes | Yes | — | No | No |
| Physical topological hardware | **No** | — | — | — | **No** |
| n>4 braid matrices | No | — | — | — | — |
| Full Solovay–Kitaev braid search | No | — | — | — | — |
| **Later** | | | | | |
| Agent / LLM controller | No | — | — | — | — |
| Hardware adapters | No | — | — | — | — |

**Honest statements**
- All topological results are from the idealized mathematical Fibonacci anyon model.
- No claim of physical topological quantum hardware is made.
- Braid matrices for n≤4 are constructed from F/R data; larger n not yet supported.
- Logical H is explicitly approximate; error is reported, never hidden.
