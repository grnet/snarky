use std::time::Instant;
use polynomials::Univariate;
use backend::scalar;

fn main() {
    println!("-------------------");

    let poly = Univariate::create_from_u64(&vec![1, 2, 3]);
    let result = poly.evaluate(&666_u64).unwrap();

    // Time evaluation for a polynomial over the backend with degree 10 ^ 6

    let x = scalar!(666_u64); // Forces coeffs of poly to be of type backend::Scalar
    let poly = Univariate::create_from_u64(&(0..10_u64.pow(6) + 1).collect());
    let start = Instant::now();
    let res = poly.evaluate(&x).unwrap();
    println!("[+] Evaluation for polynomial of degree 10 ^ 6: {:.2?}", start.elapsed());
    println!("{}", res.0);
}
