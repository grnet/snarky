use std::ops::Add;
use std::ops::Mul;
use std::ops::AddAssign;
use std::ops::MulAssign;
use core::default::Default;
use core::convert::From;
use util::SnarkyError;

use ark_ff;
use ark_poly;
use ark_poly::Polynomial;
use ark_poly::UVPolynomial;
use ark_poly::univariate::DensePolynomial;


#[derive(PartialEq, Debug)]
pub struct Univariate<F: ark_ff::Field> {
    pub _poly: DensePolynomial::<F>,
    pub degree: isize,  // TODO: Explain why degree should not be deg(_poly)
}

impl<F: ark_ff::Field> Univariate<F> {

    pub fn create(coeffs: Vec<F>) -> Self {
        Self {
            degree: coeffs.len() as isize - 1,
            _poly: DensePolynomial::from_coefficients_vec(coeffs),
        }
    }

    pub fn degree(&self) -> isize {
        self.degree     // TODO: Return usize?
    }

    pub fn coeff(&self, i: usize) -> F {
        // TODO: Explain (arkworks truncates leading zeros)
        // TODO: Guard against i >= degree?
        
        match self._poly.coeffs.len() {
            0 => F::zero(),
            1 => {
                if i == 0 {
                    self._poly.coeffs[i]
                } else {
                    F::zero()
                }
            },
            _ => {
                if i <= self._poly.degree() {
                    self._poly.coeffs[i]
                } else {
                    F::zero()
                }
            },
        }
    }

    pub fn evaluate(&self, elm: &F) -> F {
        self._poly.evaluate(&elm)
    }
}

impl<F: ark_ff::Field + From<u64>> Univariate<F> {

    pub fn create_from_u64(coeffs: &Vec<u64>) -> Self {
        let coeffs = coeffs
            .iter()
            .map(|&c| F::from(c))
            .collect::<Vec<_>>();
        Self::create(coeffs)
    }
}
