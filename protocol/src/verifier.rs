pub use crate::srs::{Trapdoor, SRS};
pub use crate::prover::BatchProof;
use circuits::ConstraintSystem;
use crate::updater::Phase;

use backend::*;
use ark_ec::{AffineCurve, PairingEngine};
use num_traits::identities::Zero;
use ark_std::rand::Rng as ArkRng;

use rayon::prelude::*;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Verification {
    FAILURE = 0,
    SUCCESS = 1,
}

impl From<Verification> for bool {
    #[inline]
    fn from(source: Verification) -> bool {
        match source {
            Verification::FAILURE => false,
            Verification::SUCCESS => true,
        }
    }
}

impl From<bool> for Verification {
    #[inline]
    fn from(source: bool) -> Verification {
        match source {
            false   => Verification::FAILURE,
            true    => Verification::SUCCESS,
        }
    }
}

// No batching; use only for testing
pub fn verify_naive(qap: &ConstraintSystem, srs: &SRS, batch: &BatchProof) -> Verification {
    let (m, n, l) = qap.shape();
    let (u, v, w, t) = qap.collections();
    let G = genG1!();
    let H = genG2!();

    // step 1
    let srs_u = &srs.u;
    let srs_s = &srs.s;

    // step 2
    let out_a = srs.check_u(&qap).unwrap_or(false);

    // step 3-4
    let out_b = batch.verify_naive(&srs, Phase::ONE).unwrap_or(false);

    // step 5
    let out_c = (1..2 * n - 1)
        .into_par_iter()
        .map(|i| {
            ct_eq!(pair!(srs_u.0[i].0, H), pair!(G, srs_u.0[i].1)) &
            ct_eq!(pair!(srs_u.0[i].0, H), pair!(srs_u.0[i - 1].0, srs_u.0[1].1))
        })
        .reduce(|| true, |acc, b| acc & b);
    
    // step 6
    let out_d = (0..n) 
        .into_par_iter()
        .map(|i| {
            ct_eq!(pair!(srs_u.1[i].0, H), pair!(G, srs_u.1[i].2)) &
            ct_eq!(pair!(srs_u.1[i].0, H), pair!(srs_u.0[i].0, srs_u.1[0].2)) &
            ct_eq!(pair!(srs_u.1[i].1, H), pair!(G, srs_u.1[i].3)) &
            ct_eq!(pair!(srs_u.1[i].1, H), pair!(srs_u.0[i].0, srs_u.1[0].3))
        })
        .reduce(|| true, |acc, b| acc & b);
    

    // step 7
    let out_e = srs.check_s(&qap).unwrap_or(false);

    // step 8-9
    let out_f = batch.verify_naive(&srs, Phase::TWO).unwrap_or(false);

    // step 10
    let out_g = (0..m - l)
        .into_par_iter()
        .map(|i| {
            let s_i = (0..n)
                .into_par_iter()
                .map(|j| add1!(
                    smul1!(u[i].coeff(j), srs_u.1[j].1),
                    smul1!(v[i].coeff(j), srs_u.1[j].0),
                    smul1!(w[i].coeff(j), srs_u.0[j].0)
                ))
                .reduce(|| zeroG1!(), |acc, inc| add1!(acc, inc));
            ct_eq!(pair!(srs_s.2[i], srs_s.1), pair!(s_i, H))
        })
        .reduce(|| true, |acc, b| acc & b);

    // step 11
    let out_h = {
        let Gt = (0..n - 1)
            .into_par_iter()
            .map(|j| smul1!(t.coeff(j), srs_u.0[j].0))
            .reduce(|| zeroG1!(), |acc, inc| add1!(acc, inc));
        (0..n - 1)
            .into_par_iter()
            .map(|i| {
                ct_eq!(pair!(srs_s.3[i], srs_s.1), pair!(Gt, srs_u.0[i].1))
            })
            .reduce(|| true, |acc, b| acc & b)
    };


    Verification::from({
        out_a & 
        out_b & 
        out_c & 
        out_d & 
        out_e & 
        out_f & 
        out_g & 
        out_h
    })
}


pub fn verify(qap: &ConstraintSystem, srs: &SRS, batch: &BatchProof) -> Verification {
    let (m, n, l) = qap.shape();
    let (u, v, w, t) = qap.collections();
    let G = genG1!();
    let H = genG2!();

    // step 1
    let srs_u = &srs.u;
    let srs_s = &srs.s;

    // step 2
    let out_a = srs.check_u(&qap).unwrap_or(false);

    // step 3
    let aux = [2 * n - 2, m, batch.batch_1.len(), batch.batch_2.len()];
    let max = *aux.iter().max().unwrap();
    let s = (0..max + 1)
        .into_par_iter()
        .map(|_| rscalar!(::util::snarky_rng()))
        .collect::<Vec::<backend::Scalar>>();

    // step 4-6
    let out_b = batch.verify(&srs, &s, Phase::ONE).unwrap_or(false);

    // step 7
    let (A, B, C) = (1..2 * n - 1)
        .into_par_iter()
        .map(|i| (
            smul1!(s[i], srs_u.0[i].0),
            smul2!(s[i], srs_u.0[i].1),
            smul1!(s[i], srs_u.0[i -  1].0)
        ))
        .reduce(|| (zeroG1!(), zeroG2!(), zeroG1!()),
            |
                (a1, b1, c1), 
                (a2, b2, c2)
            | 
            (
                a1 + a2, 
                b1 + b2, 
                c1 + c2,
            )
        );
    let out_c = ct_eq!(pair!(A, H), pair!(G, B)) & 
                ct_eq!(pair!(A, H), pair!(C, srs_u.0[1].1));
    
    // step 8
    let (A, B, C, D, E) = (0..n)
        .into_par_iter()
        .map(|i| (
            smul1!(s[i], srs_u.0[i].0),
            smul1!(s[i], srs_u.1[i].0),
            smul1!(s[i], srs_u.1[i].1),
            smul2!(s[i], srs_u.1[i].2),
            smul2!(s[i], srs_u.1[i].3),
        ))
        .reduce(|| (zeroG1!(), zeroG1!(), zeroG1!(), zeroG2!(), zeroG2!()),
            |
                (a1, b1, c1, d1, e1), 
                (a2, b2, c2, d2, e2)
            | 
            (
                a1 + a2, 
                b1 + b2, 
                c1 + c2, 
                d1 + d2, 
                e1 + e2,
            )
        );
    let out_d = ct_eq!(pair!(B, H), pair!(G, D)) &
                ct_eq!(pair!(B, H), pair!(A, srs_u.1[0].2)) &
                ct_eq!(pair!(C, H), pair!(G, E)) &
                ct_eq!(pair!(C, H), pair!(A, srs_u.1[0].3));

    // step 9
    let out_e = srs.check_s(&qap).unwrap_or(false);

    // step 10-12
    let out_f = batch.verify(&srs, &s, Phase::TWO).unwrap_or(false);

    // step 13
    let (A, B) = (0..m - l)
        .into_par_iter()
        .map(|i| {
            let sum = (0..n)
                .into_par_iter()
                .map(|j| {
                    smul1!(u[i].coeff(j), srs_u.1[j].1) +
                    smul1!(v[i].coeff(j), srs_u.1[j].0) +
                    smul1!(w[i].coeff(j), srs_u.0[j].0)
                })
                .reduce(|| zeroG1!(), |acc, inc| acc + inc);
            (
                smul1!(s[i], srs_s.2[i]),
                smul1!(s[i], sum),
            )
        })
        .reduce(|| (zeroG1!(), zeroG1!()),
            |
                (a1, b1),
                (a2, b2),
            | 
            (a1 + a2, b1 + b2)
        );
    let out_g = ct_eq!(pair!(A, srs_s.1), pair!(B, H));

    // step 14
    let (Gt, A, B) = (0..n - 1)
        .into_par_iter()
        .map(|i| (
            smul1!(t.coeff(i), srs_u.0[i].0),
            smul1!(s[i], srs_s.3[i]),
            smul2!(s[i], srs_u.0[i].1),
        ))
        .reduce(|| (zeroG1!(), zeroG1!(), zeroG2!()),
            |
                (a1, b1, c1), 
                (a2, b2, c2)
            | 
            (
                a1 + a2, 
                b1 + b2, 
                c1 + c2,
            )
        );
    let out_h = ct_eq!(pair!(A, srs_s.1), pair!(Gt, B));
    

    Verification::from({
        out_a & 
        out_b & 
        out_c & 
        out_d & 
        out_e & 
        out_f & 
        out_g & 
        out_h
    })
}
