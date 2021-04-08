use crate::error::PolyError;
use crate::scalar;
use crate::backend::Scalar as F;

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
            -1 => Err(PolyError::create("Cannot evaluate: degree -1", 
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map;
    use bls12_381::{
        Scalar, G1Affine, G2Affine, pairing,
    };

    #[test]
    fn test_create() {
        let parametrization = vec! {
            vec![],
            vec![0],
            vec![0, 1],
            vec![0, 1, 2],
            vec![0, 1, 2, 3],
        };
        for coeffs in parametrization {
            assert_eq!(
                Univariate::create(
                    coeffs
                        .iter()
                        .map(|&c| scalar!(c))
                        .collect::<Vec<_>>()
                ),
                Univariate {
                    coeffs: coeffs
                        .iter()
                        .map(|&c| scalar!(c))
                        .collect::<Vec<_>>(),
                    degree: coeffs.len() as isize - 1,
                },
            )
        }
    }

    fn test_create_from_u64() {
        let parametrization = vec! {
            vec![],
            vec![0],
            vec![0, 1],
            vec![0, 1, 2],
            vec![0, 1, 2, 3],
        };
        for coeffs in parametrization {
            assert_eq!(
                Univariate::create_from_u64(&coeffs),
                Univariate {
                    coeffs: coeffs
                        .iter()
                        .map(|&c| scalar!(c))
                        .collect::<Vec<_>>(),
                    degree: coeffs.len() as isize - 1,
                },
            )
        }
    }

    #[test]
    fn test_degree() {
        let parametrization = map! {
            vec![] => -1,
            vec![0] => 0,
            vec![0, 0] => 1, 
            vec![0, 0, 0] => 2, 
            vec![0, 0, 0, 0] => 3
        };
        for (coeffs, degree) in parametrization {
            let poly = Univariate::create_from_u64(&coeffs);
            assert_eq!(poly.degree(), degree);
        }
    }

    #[test]
    fn test_coeff() {
        let parametrization = vec! {
            vec![0],
            vec![0, 1],
            vec![0, 1, 2],
            vec![0, 1, 2, 3],
        };
        for coeffs in parametrization {
            let poly = Univariate::create_from_u64(&coeffs);
            for i in 0..coeffs.len() {
                assert_eq!(poly.coeff(i), scalar!(i as u64));
            }
        }
    }

    #[test]
    fn test_eval() {

        let edge = Univariate::create(vec![]);  // degree -1

        let parametrization = map! {
            (vec![0], 0) => 0,
            (vec![0], 1) => 0,
            (vec![1], 0) => 1,
            (vec![1], 1) => 1,
            (vec![1, 0], 0) => 1,
            (vec![1, 0], 1) => 1,
            (vec![0, 1], 0) => 0,
            (vec![0, 1], 1) => 1,
            (vec![1, 1], 0) => 1,
            (vec![1, 1], 1) => 2,
            (vec![1, 1], 2) => 3,
            (vec![1, 2, 3], 0) => 1,
            (vec![1, 2, 3], 1) => 6,
            (vec![1, 2, 3], 666) => 1332001
        };
        for ((coeffs, elm), value) in parametrization {

            // Degree -1 edge case
            assert_eq!(edge.evaluate(&scalar!(elm)).unwrap_err().code, 201);

            // Normal case
            let poly = Univariate::create_from_u64(&coeffs);
            assert_eq!(poly.evaluate(&scalar!(elm)).unwrap(), scalar!(value));
        }
    }
}
