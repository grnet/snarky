use backend::*;
use polynomials::Univariate;
use circuits::QAP;
use crate::batch::Witness;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Trapdoor(
    pub Scalar,
    pub Scalar,
    pub Scalar,
    pub Scalar,
);

impl Trapdoor {

    pub fn from_u64(a: u64, b: u64, d: u64, x: u64) -> Self {
        Self(scalar!(a), scalar!(b), scalar!(d), scalar!(x))
    }

    pub fn from_units() -> Self {
        Self(one!(), one!(), one!(), one!())
    }

    pub fn from_random(rng: &mut ::rand::RngCore) -> Self {
        Self(
            rscalar!(rng),
            rscalar!(rng),
            rscalar!(rng),
            rscalar!(rng),
        )
    }

    pub fn extract(&self) -> (Scalar, Scalar, Scalar, Scalar) {
        (self.0, self.1, self.2, self.3)
    }
}


type G1 = G1Elem;
type G2 = G2Elem;
pub type U = (Vec<(G1, G2)>, Vec<(G1, G1, G2, G2)>);
pub type S = (G1, G2, Vec<G1>, Vec<G1>);

#[derive(Clone, Debug, PartialEq)]
pub struct SRS {
    pub u: U,
    pub s: S,
}

impl SRS {

    pub fn setup_with_unit_trapdoor(qap: &QAP) -> (Self, Trapdoor) {
        SRS::setup(&qap, Some(Trapdoor::from_units()))
    }

    pub fn setup_with_random_trapdoor(qap: &QAP) -> (Self, Trapdoor) {
        SRS::setup(&qap, None)
    }

    pub fn setup(qap: &QAP, trapdoor: Option::<Trapdoor>) -> (Self, Trapdoor) {
        let trp = match trapdoor {
            Some(trp) => trp,
            None => {
                let mut rng = rand::thread_rng();
                Trapdoor::from_random(&mut rng)
            }
        };
        let srs = SRS::create(&trp, &qap);
        (srs, trp)
    }

    pub fn create(trp: &Trapdoor, qap: &QAP) -> Self {
        Self {
            u: Self::create_u(&trp, &qap),
            s: Self::create_s(&trp, &qap),
        }
    }

    fn create_u(trp: &Trapdoor, qap: &QAP) -> U {
        let (a, b, _, x) = trp.extract();
        let (_, n, _) = qap.shape();

        let G = genG1!();
        let H = genG2!();

        let c1 = (0..2 * n - 1)
            .map(|i| {
                let res = (
                    smul1!(pow!(x, i), G),
                    smul2!(pow!(x, i), H),
                );
                res
            })
            .collect();

        let c2 = (0..n)
            .map(|i| {
                let res = (
                    smul1!(a * pow!(x, i), G),
                    smul1!(b * pow!(x, i), G),
                    smul2!(a * pow!(x, i), H),
                    smul2!(b * pow!(x, i), H),
                );
                res
            })
            .collect();

        (c1, c2)
    }

    fn create_s(trp: &Trapdoor, qap: &QAP) -> S {
        let (a, b, d, x) = trp.extract();
        let (m, n, l) = qap.shape();
        let (u, v, w, t) = qap.collections();

        let G = genG1!();
        let H = genG2!();

        let dinv = inv!(d);

        let c1 = smul1!(d, G);
        let c2 = smul2!(d, H);

        let c3 = (l + 1..m + 1)
            .map(|i| {
                let ux_i = u[i].evaluate(&x).unwrap();
                let vx_i = v[i].evaluate(&x).unwrap();
                let wx_i = w[i].evaluate(&x).unwrap();
                smul1!((b * ux_i + a * vx_i + wx_i) * dinv, G)
            })
            .collect();

        let tx = t.evaluate(&x).unwrap();
        let c4 = (0..n - 1)
            .map(|i| smul1!(pow!(x, i) * tx * dinv, G))
            .collect();

        (c1, c2, c3, c4)
    }

    pub fn update(&mut self, qap: &QAP, w: Witness) {
        let (m, n, l) = qap.shape();
        match w {
            Witness::ONE(a, b, x) => {
                let srs_u = &self.u;

                // phase 1, step 8 (recompute u-component)
                let c1 = (0..2 * n - 1)
                    .map(|i| {
                        let res = (
                            smul1!(pow!(x, i), srs_u.0[i].0),
                            smul2!(pow!(x, i), srs_u.0[i].1),
                        );
                        res
                    })
                    .collect();
                let c2 = (0..n)
                    .map(|i| {
                        let res = (
                            smul1!(a * pow!(x, i), srs_u.1[i].0),
                            smul1!(b * pow!(x, i), srs_u.1[i].1),
                            smul2!(a * pow!(x, i), srs_u.1[i].2),
                            smul2!(b * pow!(x, i), srs_u.1[i].3),
                        );
                        res
                    }) 
                    .collect();
                let u_new: U =  (c1, c2);

                // phase 1, step 9 (recompute s-component)
                let s_new = SRS::specialize(&qap, &u_new);

                // phase 1, step 10
                self.u = u_new;
                self.s = s_new;
            },
            Witness::TWO(d) => {
                let srs_s = &self.s;

                // phase 2, step 5  (recompute s-component)
                let dinv = inv!(d);
                let c1 = smul1!(d, srs_s.0);
                let c2 = smul2!(d, srs_s.1);
                let c3 = (0..m - l)
                    .map(|i| smul1!(dinv, srs_s.2[i]))
                    .collect();
                let c4 = (0..n - 1)
                    .map(|i| smul1!(dinv, srs_s.3[i]))
                    .collect();
                self.s = (c1, c2, c3, c4)
            },
        }
    }

    fn specialize(qap: &QAP, srs_u: &U) -> S {
        let (m, n, l) = qap.shape();
        let (u, v, w, t) = qap.collections();
        let zero = zeroG1!();

        let c1 = genG1!();
        let c2 = genG2!();
        let c3 = (0..m - l)
            .map(|i| {
                let mut s_i = zero;
                for j in 0..n {
                    s_i = add1!(
                        s_i,
                        add1!(
                            smul1!(u[i].coeff(j), srs_u.1[j].1),
                            smul1!(v[i].coeff(j), srs_u.1[j].0),
                            smul1!(w[i].coeff(j), srs_u.0[j].0)
                        )
                    );
                }
                s_i
            })
            .collect();
        let c4 = (0..n - 1)
            .map(|i| {
                let mut s_i = zero;
                for j in 0..n {
                    s_i = add1!(
                        s_i,
                        smul1!(t.coeff(j), srs_u.0[i + j].0)
                    );
                }
                s_i
            })
            .collect();

        (c1, c2, c3, c4)
    }

    pub fn check_u(&self, qap: &QAP) -> bool {
        let (_, n, _) = qap.shape();
        let srs_u = &self.u;
        // verification: step 2
        if !(
            srs_u.0.len() == 2 * n - 1 && 
            srs_u.1.len() == n
        ) 
        {
            return false
        }
        for i in 0..2 * n - 1 {
            match
                contained_in_group!(srs_u.0[i].0) &&
                contained_in_group!(srs_u.0[i].1)
            {
                true    => continue,
                _       => return false
            }
        }
        for i in 0..n {
            match
                contained_in_group!(srs_u.1[i].0) &&
                contained_in_group!(srs_u.1[i].1) &&
                contained_in_group!(srs_u.1[i].2) &&
                contained_in_group!(srs_u.1[i].3)
            {
                true    => continue,
                _       => return false
            }
        }
        true
    }

    pub fn check_s(&self, qap: &QAP) -> bool {
        let (m, n, l) = qap.shape();
        let srs_s = &self.s;
        // verification: step 7
        if !(
            contained_in_group!(srs_s.0) &&
            contained_in_group!(srs_s.1) &&
            srs_s.2.len() == m - l &&
            srs_s.3.len() == n - 1
        ) 
        {
            return false
        }
        for i in 0..m - l {
            match
                contained_in_group!(srs_s.2[i])
            {
                true    => continue,
                _       => return false
            }
        }
        for i in 0..n - 1 {
            match
                contained_in_group!(srs_s.3[i])
            {
                true    => continue,
                _       => return false
            }
        }
        true
    }
}
