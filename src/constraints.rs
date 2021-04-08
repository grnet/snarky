use crate::polynomial::Univariate;

#[derive(Debug, PartialEq)]
pub struct QAP {
    m: usize,
    n: usize,
    l: usize,
    u: Vec<Univariate>,
    v: Vec<Univariate>,
    w: Vec<Univariate>,
    t: Univariate,
}

use crate::error::QAPError;

impl QAP {

    pub fn create(u: Vec<Univariate>, v: Vec<Univariate>, w: Vec<Univariate>, 
        t: Univariate, l: usize) -> Result<Self, QAPError> {
        let m = u.len() - 1;
        if v.len() != m + 1 || w.len() != m + 1 {
            let line = line!() - 1;
            Err(QAPError::create("Unequal lengths for u, v, w", file!(), line, 101))
        } else if l + 1 > m {
            let line = line!() - 1;
            Err(QAPError::create("l is not < m", file!(), line, 102))
        } else {
            let n = t.degree() as usize;
            let mut line = 0;
            for p in [&u, &v, &w].iter() {
                for i in 0..m + 1 {
                    if p[i].degree() as usize != n - 1 {
                        line = line!() - 1;
                        return Err(QAPError::create(
                            "Detected degree unequal to n-1", 
                            file!(), 
                            line,
                            103,
                        ))
                    }
                }
            }
            Ok(Self { m, n, l, u, v, w, t })
        }
    }

    pub fn create_default(m: usize, n: usize, l: usize) -> Result<Self, QAPError> {

        let mut coeffs1 = vec![1];
        coeffs1.append(&mut vec![0; n - 1]); // [1] + (n - 1) * [0]
        let u = vec![Univariate::create_from_u64(&coeffs1); m + 1];
        let v = vec![Univariate::create_from_u64(&coeffs1); m + 1];
        let w = vec![Univariate::create_from_u64(&coeffs1); m + 1];

        let mut coeffs2 = vec![1];
        coeffs2.append(&mut vec![0; n]);        // [1] + n * [0]
        let t = Univariate::create_from_u64(&coeffs2);

        Self::create(u, v, w, t, l)
    }

    pub fn dimensions(&self) -> (usize, usize, usize) {
        let m = self.m;
        let n = self.n;
        let l = self.l;
        (m, n, l)
    }

    pub fn collections(&self) -> 
        (&Vec<Univariate>, &Vec<Univariate>, &Vec<Univariate>, &Univariate) 
    {
        let u = &self.u;
        let v = &self.v;
        let w = &self.w;
        let t = &self.t;
        (u, v, w, t)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_QAPError_with_code_101() {
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
    fn test_QAPError_with_code_102() {
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
    fn test_QAPError_with_code_103() {
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
    fn test_QAPError_upon_default_creation() {
        let (m, n, l) = (3, 4, 3);
        let result = QAP::create_default(m, n, l);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, 102);
    }
}
