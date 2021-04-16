mod error;

use crate::error::PolyError;
use backend::scalar;
use backend::Scalar as F;

#[derive(Clone, PartialEq, Debug)]
pub struct Univariate {
    pub coeffs: Vec<F>,
    pub degree: isize,
}

impl Univariate {

    pub fn create(coeffs: Vec<F>) -> Self {
        Self {
            degree: coeffs.len() as isize - 1,
            coeffs: coeffs,
        }
    }

    pub fn create_from_u64(coeffs: &Vec<u64>) -> Self {
        let coeffs = coeffs
            .iter()
            .map(|&c| scalar!(c))
            .collect::<Vec<_>>();
        Self::create(coeffs)
    }

    // Horner's method NOTE: Sparse polynomial evaluation can be
    // more efficient with exponentiation optimized with 
    // square-and-add method which is log(N).
    pub fn evaluate(&self, elm: &F) -> Result<F, PolyError> {
        match self.degree {
            -1 => Err(
                PolyError::create("Cannot evaluate: degree -1", 
                    file!(), 
                    line!() - 4, 
                    201
                )),
            _  => {
                let mut result = F::zero();
                if self.coeffs.len() > 0 {
                    let n = self.coeffs.len() - 1;
                    result = self.coeffs[n];
                    for i in 0..n {
                        result *= elm;
                        result += &self.coeffs[n - i - 1];
                    }
                }
                Ok(result)
            }
        }
    }

    pub fn degree(&self) -> isize {
        self.degree
    }

    pub fn coeff(&self, i: usize) -> F {
        self.coeffs[i]
    }
}

