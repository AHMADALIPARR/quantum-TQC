// Rust implementation of the Truncated phi^4 Operator
use nalgebra::{DMatrix, ComplexField};

struct Phi4Oscillator {
    dim: usize,
    lambda: f64,
}

impl Phi4Oscillator {
    fn construct_hamiltonian(&self) -> DMatrix<f64> {
        let mut a = DMatrix::zeros(self.dim, self.dim);
        let mut a_dag = DMatrix::zeros(self.dim, self.dim);

        for i in 0..self.dim - 1 {
            let val = ((i + 1) as f64).sqrt();
            a[(i, i + 1)] = val;
            a_dag[(i + 1, i)] = val;
        }

        let phi = (&a + &a_dag) * (0.5f64).sqrt();
        let phi2 = &phi * &phi;
        let phi4 = &phi2 * &phi2;

        phi4 * (self.lambda / 24.0)
    }
}

fn main() {
    let osc = Phi4Oscillator { dim: 4, lambda: 1.0 };
    let h_int = osc.construct_hamiltonian();
    println!("Truncated phi^4 Matrix:\n{}", h_int);
}
