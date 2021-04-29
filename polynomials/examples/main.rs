use std::time::Instant;
use polynomials::Univariate;
use backend::scalar;
use ark_poly::univariate::{SparsePolynomial, DensePolynomial};

fn main() {

    let EXP = 7;

    println!("-------------------");

    // Time evaluation for a polynomial over the backend with degree 10 ^ 6

    let x = scalar!(666_u64); // Forces coeffs of poly to be of type backend::Scalar
    let poly = Univariate::create_from_u64(
        &(0..10_u64.pow(EXP) + 1)
        .map(|i| match i % 2 {
            0 => 0_u64,
            _ => i
        })
        .collect()
    );
    let start = Instant::now();
    // let res = poly.evaluate(&x).unwrap();
    let res = poly.evaluate(&x);
    println!("[+] (wrapped) Evaluation for polynomial of degree 10 ^ 6: {:.2?}", start.elapsed());
    println!("{}", res.0);

    use ark_poly::Polynomial;
    // let v: Vec<(usize, backend::Scalar)> = (0..10u64.pow(EXP) + 1)
    //     .map(|i| (i as usize, match i % 2 {
    //         0 => scalar!(0 as u64),
    //         _ => scalar!(i)
    //     }))
    //     .collect();
    // let poly = SparsePolynomial::from_coefficients_slice(
    //     v.as_slice()
    // );
    // let start = Instant::now();
    // let res = poly.evaluate(&x);
    // println!("[+] (sparse) Evaluation for polynomial of degree 10 ^ 6: {:.2?}", start.elapsed());
    // println!("{}", res.0);

    // use ark_poly::Polynomial;
    use ark_poly::UVPolynomial;
    let v: Vec<backend::Scalar> = (0..10u64.pow(EXP) + 1)
        .map(|i| match i % 2 {
            0 => scalar!(0 as u64),
            _ => scalar!(i)
        })
        .collect();
    let poly = DensePolynomial::<backend::Scalar>
        ::from_coefficients_slice(
            v.as_slice()
        );
    let start = Instant::now();
    let res = poly.evaluate(&x);
    println!("[+] (dense) Evaluation for polynomial of degree 10 ^ 6: {:.2?}", start.elapsed());
    println!("{}", res.0);

}
