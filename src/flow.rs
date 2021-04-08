use crate::{
    one, zero, scalar, pow, contained_in_group, G1_gen, G2_gen, G1_zero, G2_zero, 
    add_1, add_2, mult_1, mult_2, pair};
use crate::backend::{Scalar,
    G1Elem as G1, 
    G2Elem as G2,
};
use crate::constraints::QAP;

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

type U = (Vec<(G1, G2)>, Vec<(G1, G1, G2, G2)>);
type S = (G1, G2, Vec<G1>, Vec<G1>);

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
        let (_, n, _) = qap.dimensions();

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
    if !(srs_u.0.len() == 2 * n - 1 && srs_u.1.len() == n) {
        return Verification::FAILURE
    }
    for i in 0..2 * n - 1 {
        match
            contained_in_group!(srs_u.0[i].0) &&
            contained_in_group!(srs_u.0[i].1)
        {
            true    => continue,
            _       => return Verification::FAILURE
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
            _       => return Verification::FAILURE
        }
    }

    // step 3
    // step 4
    
    // step 5
    for i in 1..2 * n - 1 {
        match 
            pair!(srs_u.0[i].0, H) == pair!(G, srs_u.0[i].1) &&
            pair!(srs_u.0[i].0, H) == pair!(srs_u.0[i - 1].0, srs_u.0[1].1)
        {
            true    => continue,
            _       => return Verification::FAILURE
        }
    }

    // step 6
    for i in 0..n {
        match
            pair!(srs_u.1[i].0, H) == pair!(G, srs_u.1[i].2) &&
            pair!(srs_u.1[i].0, H) == pair!(srs_u.0[i].0, srs_u.1[0].2) &&
            pair!(srs_u.1[i].1, H) == pair!(G, srs_u.1[i].3) &&
            pair!(srs_u.1[i].1, H) == pair!(srs_u.0[i].0, srs_u.1[0].3)
        {
            true    => continue,
            _       => return Verification::FAILURE
        }
    }

    // step ~7
    if !(
        contained_in_group!(srs_s.0) && 
        contained_in_group!(srs_s.1)
    ) {
        return Verification::FAILURE
    }

    // step 8

    // ~step 9 
    if !(pair!(srs_s.0, H) == pair!(G, srs_s.1)) {
        return Verification::FAILURE
    }

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
        match
            pair!(srs_s.2[i], srs_s.1) == pair!(s_i, H)
        {
            true    => continue,
            _       => return Verification::FAILURE
        }
    }

    // step 11
    let mut Gt = G1_zero!();
    for j in 0..n - 1 {
        Gt = add_1!(Gt, mult_1!(srs_u.0[j].0, t.coeff(j)));
    }
    for i in 0..n - 1 {
        match
            pair!(srs_s.3[i], srs_s.1) == pair!(Gt, srs_u.0[i].1)
        {
            true    => continue,
            _       => return Verification::FAILURE
        }
    }
    
    Verification::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
}
