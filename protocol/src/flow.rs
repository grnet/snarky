use backend::*;
use circuits::QAP;
use crate::srs::{U, S};
pub use crate::srs::{Trapdoor, SRS};
pub use crate::batch::BatchProof;
use crate::prover::RhoProof;
use crate::batch::{Witness, UpdateProof};

use rand::RngCore;          // Must be in scope for update
use subtle::ConstantTimeEq; // Must be in scope for ct equality checks


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

pub fn update(qap: &QAP, srs: &mut SRS, batch: &mut BatchProof, phase: Phase) {
    let mut rng = rand::thread_rng();
    match phase {
        Phase::ONE => {
            let witness = Witness::ONE(
                rscalar!(rng), 
                rscalar!(rng), 
                rscalar!(rng),
            );                                          // phase 1: step 2
            batch.append(UpdateProof::create(
                &srs, 
                &witness
            ));                                         // phase 1: steps 3-6, 7
            srs.update(&qap, witness);                  // phase 1: steps 8-10
        },
        Phase::TWO => {
            let witness = Witness::TWO(rscalar!(rng));  // phase 2: step 2
            batch.append(UpdateProof::create(
                &srs,
                &witness,
            ));                                         // phase 2: steps 3-4
            srs.update(&qap, witness);                  // phase 2: step 5
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

    // ---------------------------------------------------------------

    // step 2
    if !srs.check_u(&qap) {
        return Verification::FAILURE
    }

    // step 3-4
    if !batch.verify(&srs, Phase::ONE) {
        return Verification::FAILURE
    }

    // step 5
    for i in 1..2 * n - 1 {
        match
            ct_eq!(pair!(srs_u.0[i].0, H), pair!(G, srs_u.0[i].1)) &&
            ct_eq!(pair!(srs_u.0[i].0, H), pair!(srs_u.0[i - 1].0, srs_u.0[1].1))
        {
            true    => continue,
            _       => return Verification::FAILURE
        }
    }

    // step 6
    for i in 0..n {
        match
            ct_eq!(pair!(srs_u.1[i].0, H), pair!(G, srs_u.1[i].2)) &&
            ct_eq!(pair!(srs_u.1[i].0, H), pair!(srs_u.0[i].0, srs_u.1[0].2)) &&
            ct_eq!(pair!(srs_u.1[i].1, H), pair!(G, srs_u.1[i].3)) &&
            ct_eq!(pair!(srs_u.1[i].1, H), pair!(srs_u.0[i].0, srs_u.1[0].3))
        {
            true    => continue,
            _       => return Verification::FAILURE
        }
    }

    // ---------------------------------------------------------------

    // step 7
    if !srs.check_s(&qap) {
        return Verification::FAILURE
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
            ct_eq!(pair!(srs_s.2[i], srs_s.1), pair!(s_i, H))
        {
            true    => continue,
            _       => return Verification::FAILURE
        }
    }

    // ----------------------------------------------------------------

    // step 11
    let mut Gt = zeroG1!();
    for j in 0..n - 1 {
        Gt = add1!(Gt, smul1!(t.coeff(j), srs_u.0[j].0));
    }
    for i in 0..n - 1 {
        match
            ct_eq!(pair!(srs_s.3[i], srs_s.1), pair!(Gt, srs_u.0[i].1))
        {
            true    => continue,
            _       => return Verification::FAILURE
        }
    }

    Verification::SUCCESS
}
