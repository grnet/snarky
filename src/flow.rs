use crate::{
    one, zero, scalar, pow, G1_gen, G2_gen, G1_zero, G2_zero, 
    add_1, add_2, mult_1, mult_2, pair};
use crate::backend::{Scalar, Univariate, 
    G1Elem as G1, 
    G2Elem as G2,
};

type U = (Vec<(G1, G2)>, Vec<(G1, G1, G2, G2)>);
type S = (G1, G2, Vec<G1>, Vec<G1>);

pub struct QAP {
    m: usize,
    n: usize,
    l: usize,
    u: Vec<Univariate>,
    v: Vec<Univariate>,
    w: Vec<Univariate>,
    t: Univariate,
}

impl QAP {

    pub fn create(
        u: Vec<Univariate>, 
        v: Vec<Univariate>, 
        w: Vec<Univariate>, 
        t: Univariate, 
        l: usize
    ) -> Result<Self, &'static str> {
        let m = u.len() - 1;
        if v.len() != m + 1 || w.len() != m + 1 {
            Err("Could not create: unequal lengths for u, v, w")
        } else if l + 1 > m {
            Err("Could not create: l is not < m")
        } else {
            let n = t.degree() as usize;
            let mut failed = false;
            'outer: for p in [&u, &v, &w].iter() {
                for i in 0..m + 1 {
                    if p[i].degree() as usize != n - 1 {
                        failed = true;
                        break 'outer;
                    }
                }
            }
            match failed {
                true => Err("Could not create: unequal lengths encountered"),
                _    => Ok(Self { m, n, l, u, v, w, t })
            }
        }
    }

    pub fn create_default(m: usize, n: usize, l: usize) -> Self {

        let mut coeffs1 = vec![1];
        coeffs1.append(&mut vec![0; n - 1]);
        let u = vec![Univariate::create_from_u64(&coeffs1); m + 1];
        let v = vec![Univariate::create_from_u64(&coeffs1); m + 1];
        let w = vec![Univariate::create_from_u64(&coeffs1); m + 1];

        let mut coeffs2 = vec![1];
        coeffs2.append(&mut vec![0; n]);
        let t = Univariate::create_from_u64(&coeffs2);

        Self::create(u, v, w, t, l).unwrap()    // TODO: Handle error
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
        let a = trapdoor.a;
        let b = trapdoor.b;
        let x = trapdoor.x;
        let n = qap.n;

        let G = G1_gen!();
        let H = G2_gen!();

        // TODO: Create and use backend agnostic 
        // pow macro after testing verification
     
        // Compute first component
        let c1 = (0..2 * n - 1)
            .map(|i| {
                let exponent = [(i as u64), 0, 0, 0];
                let res = (
                    mult_1!(G, x.pow(&exponent)),
                    mult_2!(H, x.pow(&exponent)),
                    // mult_1!(G, pow!(x, i as u64)),
                    // mult_2!(H, pow!(x, i as u64)),
                );
                res
            })
            .collect();

        // Compute second component
        let c2 = (0..n)
        // let c2 = (0..n as u64)
            .map(|i| {
                let exponent = [(i as u64), 0, 0, 0];
                let res = (
                    mult_1!(G, a * x.pow(&exponent)),
                    mult_1!(G, b * x.pow(&exponent)),
                    mult_2!(H, a * x.pow(&exponent)),
                    mult_2!(H, b * x.pow(&exponent)),
                    // mult_1!(G, a * pow!(x, i)),
                    // mult_1!(G, b * pow!(x, i)),
                    // mult_2!(H, a * pow!(x, i)),
                    // mult_2!(H, b * pow!(x, i)),
                );
                res
            })
            .collect();

        (c1, c2)
    }

    fn generate_s(trapdoor: &Trapdoor, qap: &QAP) -> S {
        let a = trapdoor.a;
        let b = trapdoor.b;
        let d = trapdoor.d;
        let x = trapdoor.x;
        let n = qap.n;
        let m = qap.m;
        let l = qap.l;
        let u = &qap.u;
        let v = &qap.v;
        let w = &qap.w;
        let t = &qap.t;

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
            .map(|i| {
                let exponent = [(i as u64), 0, 0, 0];
                mult_1!(G, x.pow(&exponent) * tx * dinv)
                // mult_1!(G, pow!(x, i as u64) * tx * dinv)
            })
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

pub fn verify(qap: &QAP, srs: &SRS) -> bool {
    let n = qap.n;
    let m = qap.m;
    let l = qap.l;
    let u = &qap.u;
    let v = &qap.v;
    let w = &qap.w;
    let t = &qap.t;
    let G = G1_gen!();
    let H = G2_gen!();

    // ~step 1
    let srs_u = &srs.u;
    let srs_s = &srs.s;

    // step 2
    assert_eq!(srs_u.0.len(), 2 * n - 1);   // TODO: Handle error
    assert_eq!(srs_u.1.len(), n);           // TODO: Handle error
    // TODO: Implement and use backend agnostic isG1Elem, isG2Elem macros
    for i in 0..2 * n - 1 {
        assert!(bool::from(srs_u.0[i].0.is_on_curve()));
        assert!(bool::from(srs_u.0[i].1.is_on_curve()));
    }
    for i in 0..n {
        assert!(bool::from(srs_u.1[i].0.is_on_curve()));
        assert!(bool::from(srs_u.1[i].1.is_on_curve()));
        assert!(bool::from(srs_u.1[i].2.is_on_curve()));
        assert!(bool::from(srs_u.1[i].3.is_on_curve()));
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
    assert!(bool::from(srs_s.0.is_on_curve()));
    assert!(bool::from(srs_s.1.is_on_curve()));

    // step 8

    // ~step 9 
    assert_eq!(pair!(srs_s.0, H), pair!(G, srs_s.1));

    // step 10
    for i in (0..m - l) {
        let mut s_i = G1_zero!();
        for j in (0..n) {
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
    for j in (0..n - 1) {
        Gt = add_1!(Gt, mult_1!(srs_u.0[j].0, t.coeff(j)));
    }
    for i in (0..n - 1) {
        assert_eq!(pair!(srs_s.3[i], srs_s.1), pair!(Gt, srs_u.0[i].1));
    }
    

    true
}
