use polynomials::Univariate;
use circuits::ConstraintSystem;
use std::iter::FromIterator;

macro_rules! create_polynomial_collections {
    ($m: expr, $n: expr, $l: expr, $default: expr) => {
        {
            let mut u = Vec::with_capacity($m + 1);
            let mut v = Vec::with_capacity($m + 1);
            let mut w = Vec::with_capacity($m + 1);
            let t;
            match $default {
                false => {
                    for i in 0..$m + 1 {
                        u.push(Univariate::create_from_u64(&Vec::from_iter(0..$n as u64 + 0)));
                        v.push(Univariate::create_from_u64(&Vec::from_iter(1..$n as u64 + 1)));
                        w.push(Univariate::create_from_u64(&Vec::from_iter(2..$n as u64 + 2)));
                    }
                    t = Univariate::create_from_u64(&Vec::from_iter(0..$n as u64 + 1));
                }
                true => {
                    let (mut coeffs1, mut coeffs2) = (vec![1], vec![1]);
                    coeffs1.append(&mut vec![0; $n - 1]);       // [1] + (n - 1) * [0]
                    coeffs2.append(&mut vec![0; $n]);           // [1] + n * [0]
                    for i in 0..$m + 1 {
                        u.push(Univariate::create_from_u64(&coeffs1));
                        v.push(Univariate::create_from_u64(&coeffs1));
                        w.push(Univariate::create_from_u64(&coeffs1));
                    }
                    t = Univariate::create_from_u64(&coeffs2);
                }
            }
            (u, v, w, t)
        }
    }
}

#[test]
fn test_QAP_creation() {
    let (m, n, l): (usize, usize, usize) = (5, 4, 3);
    let (u1, v1, w1, t1) = create_polynomial_collections!(m, n, l, false);
    let (u2, v2, w2, t2) = create_polynomial_collections!(m, n, l, false);
    assert_eq!(
        ConstraintSystem::create(u1, v1, w1, t1, l).unwrap(),
        ConstraintSystem {
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
    let (mut u, v, w, t) = create_polynomial_collections!(m, n, l, false);
    u.pop();
    let result = ConstraintSystem::create(u, v, w, t, l);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, 101);
}

#[test]
fn test_SnarkyError_with_code_102() {
    let (m, n, l) = (3, 4, 3);
    let (u, v, w, t) = create_polynomial_collections!(m, n, l, false);
    let result = ConstraintSystem::create(u, v, w, t, l);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, 102);
}

#[test]
fn test_SnarkyError_with_code_103() {
    let (m, n, l) = (5, 4, 3);
    let (mut u, v, w, t) = create_polynomial_collections!(m, n, l, false);
    u[0] = Univariate::create_from_u64(&vec![0; n + 1]);
    let result = ConstraintSystem::create(u, v, w, t, l);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, 103);
}

#[test]
fn test_QAP_default_creation() {
    let (m, n, l) = (5, 4, 3);
    let (u, v, w, t) = create_polynomial_collections!(m, n, l, true);
    assert_eq!(
        ConstraintSystem::create_default(m, n, l).unwrap(),
        ConstraintSystem { m, n, l, u, v, w, t }
    );
}

#[test]
fn test_SnarkyError_upon_default_creation() {
    let (m, n, l) = (3, 4, 3);
    let result = ConstraintSystem::create_default(m, n, l);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, 102);
}
