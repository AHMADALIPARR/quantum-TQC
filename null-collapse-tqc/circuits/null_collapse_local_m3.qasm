// Algorithm: Null-Collapse (local, t=1)
// Spec: null-collapse-tqc/spec/algorithm.txt
// m = 3; weights w = [0.5, 0.3, 0.8]
// theta[k] = 2 * arcsin(w[k]); Ry; Z braid; measure; collapsed = (c==0)

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

measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];
