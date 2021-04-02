use crate::{one, zero, scalar, G1_gen, G2_gen, G1_zero, G2_zero, mult_1, mult_2};
use crate::backend::{Scalar, Univariate, 
    G1Elem as G1, 
    G2Elem as G2,
};

type U = (Vec<(G1, G2)>, Vec<(G1, G1, G2, G2)>);
type V = (G1, G2, Vec<G1>, Vec<G1>);

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

    pub fn create(u: Vec<Univariate>, v: Vec<Univariate>, w: Vec<Univariate>, 
        t: Univariate, l: usize
    ) -> Result<Self, &'static str> 
    {
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
    v: V,
}

impl SRS {
    fn generate(trapdoor: &Trapdoor, qap: &QAP) -> Self {
        Self {
            u: Self::generate_u(&trapdoor, &qap),
            v: Self::generate_v(&trapdoor, &qap),
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
        let c_1 = (0..2 * n - 1)
            .map(|i| {
                let exponent = [(i as u64), 0, 0, 0];
                let res = (
                    mult_1!(G, x.pow(&exponent)),
                    mult_2!(H, x.pow(&exponent)),
                );
                res
            })
            .collect();

        // Compute second component
        let c_2 = (0..n)
            .map(|i| {
                let exponent = [(i as u64), 0, 0, 0];
                let res = (
                    mult_1!(G, a * x.pow(&exponent)),
                    mult_1!(G, b * x.pow(&exponent)),
                    mult_2!(H, a * x.pow(&exponent)),
                    mult_2!(H, b * x.pow(&exponent)),
                );
                res
            })
            .collect();

        (c_1, c_2)
    }

    fn generate_v(trapdoor: &Trapdoor, qap: &QAP) -> V {
        (G1_zero!(), G2_zero!(), Vec::<G1>::new(), Vec::<G1>::new())
    }
}

pub fn setup(trapdoor: &Trapdoor, qap: &QAP) -> SRS {
    SRS::generate(&trapdoor, &qap)
}

pub fn update(qap: &QAP, srs: &SRS) -> SRS {
    SRS {
        u: (Vec::<(G1, G2)>::new(), Vec::<(G1, G1, G2, G2)>::new()),
        v: (G1_zero!(), G2_zero!(), Vec::<G1>::new(), Vec::<G1>::new()),
    }
}

pub fn verify(qap: &QAP, srs: &SRS) -> bool {
    true
}
