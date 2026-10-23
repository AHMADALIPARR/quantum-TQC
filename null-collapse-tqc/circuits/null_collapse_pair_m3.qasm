// Algorithm: Null-Collapse with pairwise braid
// Spec steps 2–3: Z on each, then CNOT-H-CNOT between adjacent pairs

OPENQASM 2.0;
include "qelib1.inc";

qreg q[3];
creg c[3];

ry(1.0471975512) q[0];
ry(0.6082466944) q[1];
ry(1.8545904360) q[2];

z q[0];
z q[1];
z q[2];

cx q[0], q[1];
h q[1];
cx q[0], q[1];

cx q[1], q[2];
h q[2];
cx q[1], q[2];

measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];
