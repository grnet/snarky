use backend::{
    Scalar,
    G1Elem as G1,
    G2Elem as G2,
    scalar,
    rscalar,
    one,
    inv,
    pow,
    genG1,
    genG2,
    smul1,
    smul2,
};
use polynomials::Univariate;
use circuits::QAP;


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
}
