use crate::{
    one, zero, scalar, pow, contained_in_group, G1_gen, G2_gen, G1_zero, G2_zero, 
    add_1, add_2, mult_1, mult_2, pair};
use crate::backend::{Scalar, Univariate, 
    G1Elem as G1, 
    G2Elem as G2,
};

type U = (Vec<(G1, G2)>, Vec<(G1, G1, G2, G2)>);
type S = (G1, G2, Vec<G1>, Vec<G1>);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Verification {
    FAILURE = 0,
    SUCCESS = 1,
}

impl Verification {
    pub fn as_bool(&self) -> bool {
        match self {
            Verification::FAILURE => false,
            Verification::SUCCESS => true,
        }
    }
}


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

pub struct Trapdoor {
    a: Scalar,
    b: Scalar,
    d: Scalar,
    x: Scalar,
}

impl Trapdoor {

    pub fn create_from_units() -> Self {
        Self {
            a: one!(), 
            b: one!(), 
            d: one!(), 
            x: one!(),
        }
    }

    fn create_from_random(rng: &mut ::rand::RngCore) -> Self {
        use crate::rand_scalar;
        Self {
            a: rand_scalar!(rng), 
            b: rand_scalar!(rng), 
            d: rand_scalar!(rng), 
            x: rand_scalar!(rng),
        }
    }

    fn extract(&self) -> (Scalar, Scalar, Scalar, Scalar) {
        let a = self.a;
        let b = self.b;
        let d = self.d;
        let x = self.x;
        (a, b, d, x)
    }
}

pub struct SRS {
    u: U,
    s: S,
}

impl SRS {
    fn generate(trapdoor: &Trapdoor, qap: &QAP) -> Self {
        Self {
            u: Self::generate_u(&trapdoor, &qap),
            s: Self::generate_s(&trapdoor, &qap),
        }
    }

    fn generate_u(trapdoor: &Trapdoor, qap: &QAP) -> U {
        let (a, b, _, x) = trapdoor.extract();
        let n = qap.n;

        let G = G1_gen!();
        let H = G2_gen!();

        // Compute first component
        let c1 = (0..2 * n - 1)
            .map(|i| {
                let res = (
                    mult_1!(G, pow!(x, i)),
                    mult_2!(H, pow!(x, i)),
                );
                res
            })
            .collect();

        // Compute second component
        let c2 = (0..n)
            .map(|i| {
                let res = (
                    mult_1!(G, a * pow!(x, i)),
                    mult_1!(G, b * pow!(x, i)),
                    mult_2!(H, a * pow!(x, i)),
                    mult_2!(H, b * pow!(x, i)),
                );
                res
            })
            .collect();

        (c1, c2)
    }

    fn generate_s(trapdoor: &Trapdoor, qap: &QAP) -> S {
        let (a, b, d, x) = trapdoor.extract();
        let (m, n, l) = qap.dimensions();
        let (u, v, w, t) = qap.collections();

        let G = G1_gen!();
        let H = G2_gen!();

        let dinv = d.invert().unwrap();

        let c1 = mult_1!(G, d);
        let c2 = mult_2!(H, d);

        let c_3 = (l + 1..m + 1)
            .map(|i| {
                let ux_i = u[i].evaluate(&x).unwrap();
                let vx_i = v[i].evaluate(&x).unwrap();
                let wx_i = w[i].evaluate(&x).unwrap();
                mult_1!(G, (b * ux_i + a * vx_i + wx_i) * dinv)
            })
            .collect();

        let tx = t.evaluate(&x).unwrap();
        let c_4 = (0..n - 1)
            .map(|i| mult_1!(G, pow!(x, i) * tx * dinv))
            .collect();

        (c1, c2, c_3, c_4)
    }
}

pub fn setup(trapdoor: &Trapdoor, qap: &QAP) -> SRS {
    SRS::generate(&trapdoor, &qap)
}

pub fn update(qap: &QAP, srs: &SRS) -> SRS {
    SRS {
        u: (Vec::<(G1, G2)>::new(), Vec::<(G1, G1, G2, G2)>::new()),
        s: (G1_zero!(), G2_zero!(), Vec::<G1>::new(), Vec::<G1>::new()),
    }
}

pub fn verify(qap: &QAP, srs: &SRS) -> Verification {
    let (m, n, l) = qap.dimensions();
    let (u, v, w, t) = qap.collections();
    let G = G1_gen!();
    let H = G2_gen!();

    // ~step 1
    let srs_u = &srs.u;
    let srs_s = &srs.s;

    // step 2
    assert_eq!(srs_u.0.len(), 2 * n - 1);
    assert_eq!(srs_u.1.len(), n);
    for i in 0..2 * n - 1 {
        assert!(contained_in_group!(srs_u.0[i].0));
        assert!(contained_in_group!(srs_u.0[i].1));
    }
    for i in 0..n {
        assert!(contained_in_group!(srs_u.1[i].0));
        assert!(contained_in_group!(srs_u.1[i].1));
        assert!(contained_in_group!(srs_u.1[i].2));
        assert!(contained_in_group!(srs_u.1[i].3));
    }

    // step 3
    // step 4
    
    // step 5
    for i in 1..2 * n - 1 {
        assert_eq!(pair!(srs_u.0[i].0, H), pair!(G, srs_u.0[i].1));
        assert_eq!(pair!(srs_u.0[i].0, H), pair!(srs_u.0[i - 1].0, srs_u.0[1].1));
    }

    // step 6
    for i in 0..n {
        assert_eq!(pair!(srs_u.1[i].0, H), pair!(G, srs_u.1[i].2));
        assert_eq!(pair!(srs_u.1[i].0, H), pair!(srs_u.0[i].0, srs_u.1[0].2));
        assert_eq!(pair!(srs_u.1[i].1, H), pair!(G, srs_u.1[i].3));
        assert_eq!(pair!(srs_u.1[i].1, H), pair!(srs_u.0[i].0, srs_u.1[0].3));
    }

    // step ~7
    assert!(contained_in_group!(srs_s.0));
    assert!(contained_in_group!(srs_s.1));

    // step 8

    // ~step 9 
    assert_eq!(pair!(srs_s.0, H), pair!(G, srs_s.1));

    // step 10
    for i in 0..m - l {
        let mut s_i = G1_zero!();
        for j in 0..n {
            let tmp = add_1!(
                mult_1!(srs_u.1[j].1, u[i].coeff(j)),
                mult_1!(srs_u.1[j].0, v[i].coeff(j)),
                mult_1!(srs_u.0[j].0, w[i].coeff(j))
            );
            s_i = add_1!(s_i, tmp);
        }
        assert_eq!(pair!(srs_s.2[i], srs_s.1), pair!(s_i, H));
    }

    // step 11
    let mut Gt = G1_zero!();
    for j in 0..n - 1 {
        Gt = add_1!(Gt, mult_1!(srs_u.0[j].0, t.coeff(j)));
    }
    for i in 0..n - 1 {
        assert_eq!(pair!(srs_s.3[i], srs_s.1), pair!(Gt, srs_u.0[i].1));
    }
    
    Verification::SUCCESS
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
