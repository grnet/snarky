use polynomials::Univariate;
use circuits::QAP;
use std::iter::FromIterator;
use backend::Scalar as F;

macro_rules! create_some_polynomials {
    ($m: expr, $n: expr, $l: expr) => {
        {
            let mut u = Vec::<Univariate<F>>::with_capacity($m + 1);
            let mut v = Vec::<Univariate<F>>::with_capacity($m + 1);
            let mut w = Vec::<Univariate<F>>::with_capacity($m + 1);
            for i in 0..$m + 1 {
                u.push(Univariate::create_from_u64(&Vec::from_iter(0..$n as u64 + 0)));
                v.push(Univariate::create_from_u64(&Vec::from_iter(1..$n as u64 + 1)));
                w.push(Univariate::create_from_u64(&Vec::from_iter(2..$n as u64 + 2)));
            }
            let t = Univariate::create_from_u64(&Vec::from_iter(0..$n as u64 + 1));
            (u, v, w, t)
        }
    }
}

macro_rules! create_default_polynomials {
    ($m: expr, $n: expr, $l: expr) => {
        {
            let mut coeffs1 = vec![1];
            let mut coeffs2 = vec![1];
            coeffs1.append(&mut vec![0; $n - 1]);       // [1] + (n - 1) * [0]
            coeffs2.append(&mut vec![0; $n]);           // [1] + n * [0]
            let mut u = Vec::<Univariate<F>>::with_capacity($m + 1);
            let mut v = Vec::<Univariate<F>>::with_capacity($m + 1);
            let mut w = Vec::<Univariate<F>>::with_capacity($m + 1);
            for i in 0..$m + 1 {
                u.push(Univariate::create_from_u64(&coeffs1));
                v.push(Univariate::create_from_u64(&coeffs1));
                w.push(Univariate::create_from_u64(&coeffs1));
            }
            let t = Univariate::create_from_u64(&coeffs2);
            (u, v, w, t)
        }
    }
}

macro_rules! create_some_polynomial_pairs {
    ($m: expr, $n: expr, $l: expr) => {
        {
            let mut u1 = Vec::<Univariate<F>>::with_capacity($m + 1);
            let mut u2 = Vec::<Univariate<F>>::with_capacity($m + 1);
            let mut v1 = Vec::<Univariate<F>>::with_capacity($m + 1);
            let mut v2 = Vec::<Univariate<F>>::with_capacity($m + 1);
            let mut w1 = Vec::<Univariate<F>>::with_capacity($m + 1);
            let mut w2 = Vec::<Univariate<F>>::with_capacity($m + 1);
            for i in 0..$m + 1 {
                u1.push(Univariate::create_from_u64(&Vec::from_iter(0..$n as u64 + 0)));
                v1.push(Univariate::create_from_u64(&Vec::from_iter(1..$n as u64 + 1)));
                w1.push(Univariate::create_from_u64(&Vec::from_iter(2..$n as u64 + 2)));
                u2.push(Univariate::create_from_u64(&Vec::from_iter(0..$n as u64 + 0)));
                v2.push(Univariate::create_from_u64(&Vec::from_iter(1..$n as u64 + 1)));
                w2.push(Univariate::create_from_u64(&Vec::from_iter(2..$n as u64 + 2)));
            }
            let t1 = Univariate::create_from_u64(&Vec::from_iter(0..$n as u64 + 1));
            let t2 = Univariate::create_from_u64(&Vec::from_iter(0..$n as u64 + 1));
            (u1, u2, v1, v2, w1, w2, t1, t2)
        }
    }
}

#[test]
fn test_QAP_creation() {
    let (m, n, l): (usize, usize, usize) = (5, 4, 3);
    let (u1, u2, v1, v2, w1, w2, t1, t2) = create_some_polynomial_pairs!(m, n, l);
    assert_eq!(
        QAP::create(u1, v1, w1, t1, l).unwrap(),
        QAP {
            m, n, l, 
            u: u2,
            v: v2,
            w: w2,
            t: t2,
        }
    );
}

#[test]
fn test_SnarkyError_with_code_101() {
    let (m, n, l) = (5, 4, 3);
    let (mut u, v, w, t) = create_some_polynomials!(m, n, l);
    u.pop();
    let result = QAP::create(u, v, w, t, l);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, 101);
}

#[test]
fn test_SnarkyError_with_code_102() {
    let (m, n, l) = (3, 4, 3);
    let (u, v, w, t) = create_some_polynomials!(m, n, l);
    let result = QAP::create(u, v, w, t, l);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, 102);
}

#[test]
fn test_SnarkyError_with_code_103() {
    let (m, n, l) = (5, 4, 3);
    let (mut u, v, w, t) = create_some_polynomials!(m, n, l);
    u[0] = Univariate::create_from_u64(&vec![0; n + 1]);
    let result = QAP::create(u, v, w, t, l);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, 103);
}

#[test]
fn test_QAP_default_creation() {
    let (m, n, l) = (5, 4, 3);
    let (u, v, w, t) = create_default_polynomials!(m, n, l);
    assert_eq!(
        QAP::create_default(m, n, l).unwrap(),
        QAP { m, n, l, u, v, w, t }
    );
}

#[test]
fn test_SnarkyError_upon_default_creation() {
    let (m, n, l) = (3, 4, 3);
    let result = QAP::create_default(m, n, l);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, 102);
}
