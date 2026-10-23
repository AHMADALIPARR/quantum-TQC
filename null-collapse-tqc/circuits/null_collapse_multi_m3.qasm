// NullCollapseAll for m=3 independent pairs
// weights = [0.0, 0.5, 1.0]
// Each pair: Ry(2 arcsin w) · SearchBraid · measure

OPENQASM 2.0;
include "qelib1.inc";

qreg q[3];
creg c[3];

// ---- pair 0: w=0 → theta=0 (pure null) ----
u1(1.5707963267948966) q[0];
h q[0];
u1(1.5707963267948966) q[0];
h q[0];
u1(1.5707963267948966) q[0];

// ---- pair 1: w=0.5 → theta=π/3 ----
ry(1.0471975511965976) q[1];
u1(1.5707963267948966) q[1];
h q[1];
u1(1.5707963267948966) q[1];
h q[1];
u1(1.5707963267948966) q[1];

// ---- pair 2: w=1 → theta=π ----
ry(3.141592653589793) q[2];
u1(1.5707963267948966) q[2];
h q[2];
u1(1.5707963267948966) q[2];
h q[2];
u1(1.5707963267948966) q[2];

measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];
// collapsed[k] = (c[k] == 0)
