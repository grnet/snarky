use backend::{
    one, zero, rndscalar, scalar, pow, contained_in_group, 
    G1_gen, G2_gen, G1_zero, G2_zero, add_1, add_2, 
    mult_1, mult_2, pair};
use backend::{Scalar,
    G1Elem as G1, 
    G2Elem as G2,
};
use circuits::QAP;

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
        Self {
            a: rndscalar!(rng), 
            b: rndscalar!(rng), 
            d: rndscalar!(rng), 
            x: rndscalar!(rng),
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

#[derive(Clone, Debug, PartialEq)]
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

        let c1 = (0..2 * n - 1)
            .map(|i| {
                let res = (
                    mult_1!(G, pow!(x, i)),
                    mult_2!(H, pow!(x, i)),
                );
                res
            })
            .collect();

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Phase {
    ONE = 1,
    TWO = 2,
}

type Rho = (G1, G1, G2, G1);

#[derive(Clone, Debug, PartialEq)]
pub struct BatchProof {
    pub phase_1: Vec<[Rho; 3]>,
    pub phase_2: Vec<Rho>,
}

impl BatchProof {
    pub fn initiate() -> Self {
        Self {
            phase_1: Vec::new(), 
            phase_2: Vec::new()
        }
    }

    pub fn phase_1_append(&mut self, proof: [Rho; 3]) {
        self.phase_1.push(proof);
    }

    pub fn phase_2_append(&mut self, proof: Rho) {
        self.phase_2.push(proof);
    }
}

use rand::RngCore;                  // Must be present for update
use crate::dlog::prove_dlog;

pub fn specialize(qap: &QAP, u_comp: &U) -> S {
    let (m, n, l) = qap.dimensions();
    let (u, v, w, t) = qap.collections();

    let c1 = G1_gen!();
    let c2 = G2_gen!();
    let c3 = (0..m - l)
        .map(|i| {
            let mut s_i = G1_zero!();
            for j in 0..n {
                s_i = add_1!(
                    s_i,
                    add_1!(
                        mult_1!(u_comp.1[j].1, u[i].coeff(j)),
                        mult_1!(u_comp.1[j].0, v[i].coeff(j)),
                        mult_1!(u_comp.0[j].0, w[i].coeff(j))
                    )
                );
            }
            s_i
        })
        .collect();
    let c4 = (0..n - 1)
        .map(|i| {
            let mut s_i = G1_zero!();
            for j in 0..n {
                s_i = add_1!(
                    s_i,
                    mult_1!(u_comp.0[i + j].0, t.coeff(j))
                );
            }
            s_i
        })
        .collect();

    (c1, c2, c3, c4)
}

pub fn update(
    qap: &QAP, 
    srs: &SRS, 
    batch: &mut BatchProof, 
    phase: Phase, 
    rng: &mut RngCore
) -> SRS {
    let (G, H) = (G1_gen!(), G2_gen!());
    let (m, n, l) = qap.dimensions();
    match phase {
        Phase::ONE => {
            // step 1
            let srs_u = &srs.u;
            // step 2 (fix witnesses)
            let a_2 = rndscalar!(rng);
            let b_2 = rndscalar!(rng);
            let x_2 = rndscalar!(rng);
            // step 3
            let pi_a_2 = prove_dlog((mult_1!(G, a_2), mult_2!(H, a_2)), a_2);
            let pi_b_2 = prove_dlog((mult_1!(G, b_2), mult_2!(H, b_2)), b_2);
            let pi_x_2 = prove_dlog((mult_1!(G, x_2), mult_2!(H, x_2)), x_2);
            // step 4
            let rho_a_2 = (
                mult_1!(srs_u.1[0].0, a_2), 
                mult_1!(G, a_2),
                mult_2!(H, a_2), 
                pi_a_2,
            );
            // step 5
            let rho_b_2 = (
                mult_1!(srs_u.1[0].1, b_2), 
                mult_1!(G, b_2),
                mult_2!(H, b_2), 
                pi_b_2,
            );
            // step 6
            let rho_x_2 = (
                mult_1!(srs_u.0[1].0, x_2), 
                mult_1!(G, x_2),
                mult_2!(H, x_2), 
                pi_x_2,
            );
            // step 7
            let rho = [rho_a_2, rho_b_2, rho_x_2];
            batch.phase_1_append(rho);    // Append here instead of returning like in the paper

            // step 8 (compute u-component)
            let c1 = (0..2 * n - 1)
                .map(|i| {
                    let res = (
                        mult_1!(srs_u.0[i].0, pow!(x_2, i)),
                        mult_2!(srs_u.0[i].1, pow!(x_2, i)),
                    );
                    res
                })
                .collect();
            let c2 = (0..n)
                .map(|i| {
                    let res = (
                        mult_1!(srs_u.1[i].0, a_2 * pow!(x_2, i)),
                        mult_1!(srs_u.1[i].1, b_2 * pow!(x_2, i)),
                        mult_2!(srs_u.1[i].2, a_2 * pow!(x_2, i)),
                        mult_2!(srs_u.1[i].3, b_2 * pow!(x_2, i)),
                    );
                    res
                })
                .collect();
            let u_new: U =  (c1, c2);

            // step 9
            let s_new = specialize(&qap, &u_new);

            // step 10
            SRS {
                u: u_new,
                s: s_new,
            }
        },
        Phase::TWO => {
            // step 1
            let srs_s = &srs.s;
            // step 2 (fix witness)
            let d_2 = rndscalar!(rng);
            // step 3
            let pi_d_2 = prove_dlog((mult_1!(G, d_2), mult_2!(H, d_2)), d_2);
            // step 4
            let rho = (
                mult_1!(srs_s.0, d_2),
                mult_1!(G, d_2),
                mult_2!(H, d_2),
                pi_d_2,
            );
            batch.phase_2_append(rho);  // Append here instead of returning like in the paper

            // step 5
            let dinv = d_2.invert().unwrap();
            let c1 = mult_1!(srs_s.0, d_2);
            let c2 = mult_2!(srs_s.1, d_2);
            let c3 = (0..m - l)
                .map(|i| {
                     mult_1!(srs_s.2[i], dinv)
                })
                .collect();
            let c4 = (0..n - 1)
                .map(|i| {
                     mult_1!(srs_s.3[i], dinv)
                })
                .collect();
            SRS {
                u: srs.u.clone(),
                s: (c1, c2, c3, c4),
            }
        }
    }
}

use crate::dlog::verify_dlog;

pub fn verify(qap: &QAP, srs: &SRS, batch: &BatchProof) -> Verification {
    let (m, n, l) = qap.dimensions();
    let (u, v, w, t) = qap.collections();
    let G = G1_gen!();
    let H = G2_gen!();

    // ~step 1
    let srs_u = &srs.u;
    let srs_s = &srs.s;
    let batch_u = &batch.phase_1;
    let batch_s = &batch.phase_2;

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
    for i in 0..batch_u.len() {
        for j in 0..3 {
            let rho = batch_u[i][j];
            match verify_dlog(&G, &H, (rho.1, rho.2), rho.3) {
                true    => {
                    if i != 0 {
                        match 
                            pair!(rho.0, H) == pair!(batch_u[i - 1][j].0, rho.2) 
                        {
                            true    => continue,
                            _       => return Verification::FAILURE
                        }
                    }
                },
                _       => return Verification::FAILURE
            }
        }
    }
    
    // step 4
    let len = batch_u.len();
    if len > 0 {
        let zero = G1_zero!();
        if !(
            srs_u.0[1].0 == batch_u[len - 1][2].0 &&
            srs_u.1[0].0 == batch_u[len - 1][0].0 &&
            srs_u.1[0].1 == batch_u[len - 1][1].0 &&
            batch_u[len - 1][2].0 != zero &&
            batch_u[len - 1][0].0 != zero &&
            batch_u[len - 1][1].0 != zero
        ) 
        {
            return Verification::FAILURE
        }
    }
    
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

    // step 7
    if !(
        contained_in_group!(srs_s.0) && 
        contained_in_group!(srs_s.1) &&
        srs_s.2.len() == m - l &&
        srs_s.3.len() == n - 1
    ) {
        return Verification::FAILURE
    }
    for i in 0..m - l {
        match
            contained_in_group!(srs_s.2[i])
        {
            true    => continue,
            _       => return Verification::FAILURE
        }
    }
    for i in 0..n - 1 {
        match
            contained_in_group!(srs_s.3[i])
        {
            true    => continue,
            _       => return Verification::FAILURE
        }
    }

    // step 8
    for i in 0..batch_s.len() {
        let rho = batch_s[i];
        match verify_dlog(&G, &H, (rho.1, rho.2), rho.3) {
            true    => {
                if i != 0 {
                    match 
                        pair!(rho.0, H) == pair!(batch_s[i - 1].0, rho.2) 
                    {
                        true    => continue,
                        _       => return Verification::FAILURE
                    }
                }
            },
            _       => return Verification::FAILURE
        }
    }

    // step 9 
    if !(pair!(srs_s.0, H) == pair!(G, srs_s.1)) {
        return Verification::FAILURE
    }
    let len = batch_s.len();
    if len > 0 {
        if !(
            srs_s.0 == batch_s[len - 1].0 &&
            batch_s[len - 1].0 != G1_zero!()
        ) 
        {
            return Verification::FAILURE
        }
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
