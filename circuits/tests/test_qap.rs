use polynomials::Univariate;
use circuits::QAP;
use std::iter::FromIterator;

#[test]
fn test_QAP_creation() {
    let (m, n, l): (usize, usize, usize) = (5, 4, 3);
    assert_eq!(
        QAP::create(
            vec![Univariate::create_from_u64(&Vec::from_iter(0..n as u64)); m + 1],
            vec![Univariate::create_from_u64(&Vec::from_iter(1..n as u64 + 1)); m + 1],
            vec![Univariate::create_from_u64(&Vec::from_iter(2..n as u64 + 2)); m + 1],
            Univariate::create_from_u64(&Vec::from_iter(0..n as u64 + 1)),
            l,
        ).unwrap(),
        QAP {
            m, n, l, 
            u: vec![Univariate::create_from_u64(&Vec::from_iter(0..n as u64)); m + 1],
            v: vec![Univariate::create_from_u64(&Vec::from_iter(1..n as u64 + 1)); m + 1],
            w: vec![Univariate::create_from_u64(&Vec::from_iter(2..n as u64 + 2)); m + 1],
            t: Univariate::create_from_u64(&Vec::from_iter(0..n as u64 + 1)),
        }
    );
}

#[test]
fn test_SnarkyError_with_code_101() {
    let (m, n, l) = (5, 4, 3);
    let u = vec![Univariate::create_from_u64(&vec![0; n]); m];
    let v = vec![Univariate::create_from_u64(&vec![0; n]); m + 1];
    let w = vec![Univariate::create_from_u64(&vec![0; n]); m + 1];
    let t = Univariate::create_from_u64(&vec![0; n + 1]);

    let result = QAP::create(u, v, w, t, l);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, 101);
}

#[test]
fn test_SnarkyError_with_code_102() {
    let (m, n, l) = (3, 4, 3);
    let u = vec![Univariate::create_from_u64(&vec![0; n]); m + 1];
    let v = vec![Univariate::create_from_u64(&vec![0; n]); m + 1];
    let w = vec![Univariate::create_from_u64(&vec![0; n]); m + 1];
    let t = Univariate::create_from_u64(&vec![0; n + 1]);

    let result = QAP::create(u, v, w, t, l);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, 102);
}

#[test]
fn test_SnarkyError_with_code_103() {
    let (m, n, l) = (5, 4, 3);
    let u = vec![Univariate::create_from_u64(&vec![0; n + 1]); m + 1];
    let v = vec![Univariate::create_from_u64(&vec![0; n]); m + 1];
    let w = vec![Univariate::create_from_u64(&vec![0; n]); m + 1];
    let t = Univariate::create_from_u64(&vec![0; n + 1]);

    let result = QAP::create(u, v, w, t, l);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, 103);
}

#[test]
fn test_QAP_default_creation() {
    let (m, n, l): (usize, usize, usize) = (5, 4, 3);
    let mut coeffs1 = vec![1];
    coeffs1.append(&mut vec![0; n - 1]); // [1] + (n - 1) * [0]
    let u = vec![Univariate::create_from_u64(&coeffs1); m + 1];
    let v = vec![Univariate::create_from_u64(&coeffs1); m + 1];
    let w = vec![Univariate::create_from_u64(&coeffs1); m + 1];
    let mut coeffs2 = vec![1];
    coeffs2.append(&mut vec![0; n]);        // [1] + n * [0]
    let t = Univariate::create_from_u64(&coeffs2);

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
