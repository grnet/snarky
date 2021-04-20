use backend::{
    one, zero, rscalar, scalar, pow, contained_in_group,
    genG1, genG2, zeroG1, zeroG2, add1, add2,
    smul1, smul2, pair};
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
            a: rscalar!(rng),
            b: rscalar!(rng),
            d: rscalar!(rng),
            x: rscalar!(rng),
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
    fn create(trp: &Trapdoor, qap: &QAP) -> Self {
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

        let dinv = d.invert().unwrap();

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

pub fn setup(trp: &Trapdoor, qap: &QAP) -> SRS {
    SRS::create(&trp, &qap)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Phase {
    ONE = 1,
    TWO = 2,
}

use crate::prover::UpdateProof;

// Collections of PoKs produced during SRS update,
// corresponding to respective phases (ONE or TWO)
#[derive(Clone, Debug, PartialEq)]
pub enum Proof {
    ONE(UpdateProof, UpdateProof, UpdateProof),
    TWO(UpdateProof),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BatchProof {
    pub phase_1: Vec<[UpdateProof; 3]>,
    pub phase_2: Vec<UpdateProof>,
}

impl BatchProof {

    pub fn initiate() -> Self {
        Self {
            phase_1: Vec::new(),
            phase_2: Vec::new()
        }
    }

    pub fn append(&mut self, proof: Proof) {
        match proof {
            Proof::ONE(r1, r2, r3) => {
                self.phase_1.push([r1, r2, r3]);
            },
            Proof::TWO(r) => {
                self.phase_2.push(r);
            },
        }
    }

    pub fn verify(&self, srs: &SRS, phase: Phase) -> bool {
        let zero = zeroG1!();
        let G = genG1!();
        let H = genG2!();
        match phase {
            Phase::ONE => {
                let batch_u = &self.phase_1;
                let srs_u = &srs.u;

                // step 3
                for i in 0..batch_u.len() {
                    for j in 0..3 {
                        let rho = &batch_u[i][j];
                        let prf = if i != 0 { Some(&batch_u[i - 1][j]) } else { None };
                        match rho.verify((&G, &H), prf)
                        {
                            true    => continue,
                            _       => return false,
                        }
                    }
                }

                // step 4
                let len = batch_u.len();
                match len > 0 && !(
                    srs_u.0[1].0 == batch_u[len - 1][2].0 &&
                    srs_u.1[0].0 == batch_u[len - 1][0].0 &&
                    srs_u.1[0].1 == batch_u[len - 1][1].0 &&
                    batch_u[len - 1][2].0 != zero &&
                    batch_u[len - 1][0].0 != zero &&
                    batch_u[len - 1][1].0 != zero
                )
                {
                    true    => false,
                    _       => true,
                }
            },
            Phase::TWO => {
                let batch_s = &self.phase_2;
                let srs_s = &srs.s;

                // step 8
                for i in 0..batch_s.len() {
                    let rho = &batch_s[i];
                    let prf = if i != 0 { Some(&batch_s[i - 1]) } else { None };
                    match rho.verify((&G, &H), prf)
                    {
                        true    => continue,
                        _       => return false,
                    }
                }

                // step 9
                match pair!(srs_s.0, H) != pair!(G, srs_s.1) {
                    true    => return false,
                    false   => {
                        let len = batch_s.len();
                        match len > 0 && !(
                            srs_s.0 == batch_s[len - 1].0 &&
                            batch_s[len - 1].0 != zero
                        ) 
                        {
                            true    => false,
                            _       => true,
                        }
                    }
                }
            }
        }
    }
}

use rand::RngCore;                  // Must be present for update

pub fn specialize(qap: &QAP, srs_u: &U) -> S {
    let (m, n, l) = qap.shape();
    let (u, v, w, t) = qap.collections();

    let c1 = genG1!();
    let c2 = genG2!();
    let c3 = (0..m - l)
        .map(|i| {
            let mut s_i = zeroG1!();
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
            let mut s_i = zeroG1!();
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

pub fn update(qap: &QAP, srs: &SRS, batch: &mut BatchProof, phase: Phase) -> SRS {
    let (G, H) = (genG1!(), genG2!());
    let (m, n, l) = qap.shape();
    let mut rng = rand::thread_rng();
    match phase {
        Phase::ONE => {
            let srs_u = &srs.u; // step 1
            let (a, b, x) = (
                rscalar!(rng), 
                rscalar!(rng), 
                rscalar!(rng),
            );                  // step 2 (fix witnesses)

            // step 3-6 (PoK for values used in update)
            let rho_a = UpdateProof::for_value((&G, &H, srs_u.1[0].0), &a);
            let rho_b = UpdateProof::for_value((&G, &H, srs_u.1[0].1), &b);
            let rho_x = UpdateProof::for_value((&G, &H, srs_u.0[1].0), &x);

            // step 7
            batch.append(Proof::ONE(rho_a, rho_b, rho_x));

            // step 8 (compute u-component)
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

            // step 9
            let s_new = specialize(&qap, &u_new);

            // step 10
            SRS {
                u: u_new,
                s: s_new,
            }
        },
        Phase::TWO => {
            let srs_s = &srs.s;     // step 1
            let d = rscalar!(rng);  // step 2 (fix witnesses)

            // step 3-4 (PoK for value used in update)
            let rho = UpdateProof::for_value((&G, &H, srs_s.0), &d);
            batch.append(Proof::TWO(rho));

            // step 5
            let dinv = d.invert().unwrap();
            let c1 = smul1!(d, srs_s.0);
            let c2 = smul2!(d, srs_s.1);
            let c3 = (0..m - l)
                .map(|i| smul1!(dinv, srs_s.2[i]))
                .collect();
            let c4 = (0..n - 1)
                .map(|i| smul1!(dinv, srs_s.3[i]))
                .collect();
            SRS {
                u: srs.u.clone(),
                s: (c1, c2, c3, c4),
            }
        }
    }
}


pub fn verify(qap: &QAP, srs: &SRS, batch: &BatchProof) -> Verification {
    let (m, n, l) = qap.shape();
    let (u, v, w, t) = qap.collections();
    let G = genG1!();
    let H = genG2!();

    // step 1
    let srs_u = &srs.u;
    let srs_s = &srs.s;
    // let batch_u = &batch.phase_1;
    // let batch_s = &batch.phase_2;

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

    // step 3-4
    if !batch.verify(&srs, Phase::ONE) {
        return Verification::FAILURE
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

    // step 8-9
    if !batch.verify(&srs, Phase::TWO) {
        return Verification::FAILURE
    }

    // step 10
    for i in 0..m - l {
        let mut s_i = zeroG1!();
        for j in 0..n {
            let tmp = add1!(
                smul1!(u[i].coeff(j), srs_u.1[j].1),
                smul1!(v[i].coeff(j), srs_u.1[j].0),
                smul1!(w[i].coeff(j), srs_u.0[j].0)
            );
            s_i = add1!(s_i, tmp);
        }
        match
            pair!(srs_s.2[i], srs_s.1) == pair!(s_i, H)
        {
            true    => continue,
            _       => return Verification::FAILURE
        }
    }

    // step 11
    let mut Gt = zeroG1!();
    for j in 0..n - 1 {
        Gt = add1!(Gt, smul1!(t.coeff(j), srs_u.0[j].0));
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
