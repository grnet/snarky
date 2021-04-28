use std::ops::Add;
use std::ops::Mul;
use std::ops::AddAssign;
use std::ops::MulAssign;
use core::default::Default;
use core::convert::From;
use util::SnarkyError;

#[derive(Clone, PartialEq, Debug)]
pub struct Univariate<F> {
    pub coeffs: Vec<F>,
    pub degree: isize,
}

impl<F> Univariate<F> {

    pub fn create(coeffs: Vec<F>) -> Self {
        Self {
            degree: coeffs.len() as isize - 1,
            coeffs: coeffs,
        }
    }

    pub fn degree(&self) -> isize {
        self.degree
    }
}

impl<F: Copy> Univariate<F> {
    pub fn coeff(&self, i: usize) -> F {
        self.coeffs[i]
    }
}

impl<F: From<u64>> Univariate<F> {

    pub fn create_from_u64(coeffs: &Vec<u64>) -> Self {
        let coeffs = coeffs
            .iter()
            .map(|&c| F::from(c))
            .collect::<Vec<_>>();
        Self::create(coeffs)
    }
}

impl<F: Default + Copy + AddAssign + MulAssign + ::std::fmt::Debug> Univariate<F> {

    // Horner's method NOTE: Sparse polynomial evaluation can be
    // more efficient with exponentiation optimized with 
    // square-and-add method which is log(N).
    pub fn evaluate(&self, elm: &F) -> Result<F, SnarkyError> {
        match self.degree {
            -1 => Err(
                SnarkyError::create("Cannot evaluate polynomial", "degree -1",
                    file!(), 
                    line!() - 4, 
                    201
                )),
            _  => {
                let mut result = F::default();  // Should be zero
                if self.coeffs.len() > 0 {
                    let n = self.coeffs.len() - 1;
                    result = self.coeffs[n];
                    for i in 0..n {
                        result *= *elm;
                        result += self.coeffs[n - i - 1];
                    }
                }
                Ok(result)
            }
        }
    }
}

