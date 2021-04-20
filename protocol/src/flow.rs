use backend::{
    G1Elem as G1,
    G2Elem as G2,
    rscalar,
    inv,
    pow, 
    contained_in_group,
    genG1, 
    genG2, 
    zeroG1, 
    add1, 
    add2,
    smul1, 
    smul2, 
    pair,
};
use circuits::QAP;
use crate::srs::{U, S};
pub use crate::srs::{Trapdoor, SRS};
pub use crate::batch::BatchProof;
use crate::prover::UpdateProof;
use crate::batch::Proof;
use rand::RngCore;                  // Must be present for update


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Phase {
    ONE = 1,
    TWO = 2,
}


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

pub fn update(qap: &QAP, srs: &mut SRS, batch: &mut BatchProof, phase: Phase) {
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

            // // step 10
            srs.u = u_new;
            srs.s = s_new;
        },
        Phase::TWO => {
            let srs_s = &srs.s;     // step 1
            let d = rscalar!(rng);  // step 2 (fix witnesses)

            // step 3-4 (PoK for value used in update)
            let rho = UpdateProof::for_value((&G, &H, srs_s.0), &d);
            batch.append(Proof::TWO(rho));

            // step 5
            let dinv = inv!(d);
            let c1 = smul1!(d, srs_s.0);
            let c2 = smul2!(d, srs_s.1);
            let c3 = (0..m - l)
                .map(|i| smul1!(dinv, srs_s.2[i]))
                .collect();
            let c4 = (0..n - 1)
                .map(|i| smul1!(dinv, srs_s.3[i]))
                .collect();
            srs.s = (c1, c2, c3, c4)
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
