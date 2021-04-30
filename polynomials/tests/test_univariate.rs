use polynomials::Univariate;
use backend::{scalar, Scalar};
use util::map;

use ark_ff;
use ark_poly;
use ark_poly::Polynomial;
use ark_poly::UVPolynomial;
use ark_poly::univariate::DensePolynomial;


#[test]
fn test_create() {
    let parametrization = vec! {
        vec![],
        vec![0_u64],
        vec![0_u64, 1],
        vec![0_u64, 1, 2],
        vec![0_u64, 1, 2, 3],
    };
    for coeffs in parametrization {
        assert_eq!(
            Univariate::<Scalar>::create(
                coeffs
                    .iter()
                    .map(|&c| scalar!(c))
                    .collect::<Vec<_>>()
            ),
            Univariate::<Scalar> {
                degree: coeffs.len() as isize - 1,
                _poly: DensePolynomial::from_coefficients_vec(
                    coeffs
                        .iter()
                        .map(|&c| scalar!(c))
                        .collect::<Vec<_>>()
                )
            },
        )
    }
}

fn test_create_from_u64() {
    let parametrization = vec! {
        vec![],
        vec![0_u64],
        vec![0_u64, 1],
        vec![0_u64, 1, 2],
        vec![0_u64, 1, 2, 3],
    };
    for coeffs in parametrization {
        assert_eq!(
            Univariate::<Scalar>::create_from_u64(&coeffs),
            Univariate::<Scalar> {
                degree: coeffs.len() as isize - 1,
                _poly: DensePolynomial::from_coefficients_vec(
                    coeffs
                        .iter()
                        .map(|&c| scalar!(c))
                        .collect::<Vec<_>>()
                )
            }
        )
    }
}

#[test]
fn test_degree() {
    let parametrization = map! {
        vec![] => -1,
        vec![0_u64] => 0,
        vec![0_u64, 0] => 1, 
        vec![0_u64, 0, 0] => 2, 
        vec![0_u64, 0, 0, 0] => 3
    };
    for (coeffs, degree) in parametrization {
        let poly = Univariate::<Scalar>::create_from_u64(&coeffs);
        assert_eq!(poly.degree(), degree);
    }
}

#[test]
fn test_coeff() {
    let parametrization = vec! {
        vec![],
        vec![0_u64],
        vec![0_u64, 0_u64],
        vec![1_u64],
        vec![1_u64, 0],
        vec![0_u64, 1],
        vec![1_u64, 0, 0],  
        vec![1_u64, 0, 1],
        vec![1_u64, 1, 0],
        vec![1_u64, 1, 1],
        vec![0_u64, 1, 2],
        vec![0_u64, 1, 2, 3],
    };
    for coeffs in parametrization {
        let poly = Univariate::<Scalar>::create_from_u64(&coeffs);
        println!("{}", poly.degree());
        println!("{}", coeffs.len());
        for i in 0..coeffs.len() {
            assert_eq!(poly.coeff(i), scalar!(coeffs[i]));
        }
    }
}

#[test]
fn test_evaluate() {

    let edge = Univariate::<Scalar>::create(vec![]);  // degree -1

    let parametrization = map! {
        (vec![0_u64], 0_u64) => 0_u64,
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

        // // Degree -1 edge case
        // assert_eq!(edge.evaluate(&scalar!(elm)).unwrap_err().code, 201);

        // Normal case
        let poly = Univariate::<Scalar>::create_from_u64(&coeffs);
        assert_eq!(poly.evaluate(&scalar!(elm)), scalar!(value));
    }
}
