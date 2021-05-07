use subtle::ConstantTimeEq; // Must be in scope for ct comparisons

use backend::*;
use circuits::QAP;
use crate::prover::{RhoProof, Witness, UpdateProof};
use crate::updater::Phase;

use backend::*;
use ark_ec::AffineCurve;            // Needed for group inclusion check
use ark_ec::PairingEngine;          // Needed for pairing
use num_traits::identities::Zero;   // Needed for zero constructions
use num_traits::identities::One;    // Needed for one constructions
use ark_ff::fields::Field;          // Needed for pow
use ark_ff::ToBytes;
use ark_std::rand::Rng as ArkRng;   // Must be in scope for rscalar
use ark_bls12_381;

pub use crate::srs::{Trapdoor, SRS};
pub use crate::prover::BatchProof;

use ark_std::rand::RngCore as ArkRngCore;
use ark_std::rand::SeedableRng;

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

pub fn verify_naive(qap: &QAP, srs: &SRS, batch: &BatchProof) -> Verification {
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


pub fn verify(qap: &QAP, srs: &SRS, batch: &BatchProof) -> Verification {
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
    let A = (1..2 * n - 1)
        .map(|i| smul1!(s[i], srs_u.0[i].0))
        .reduce(|acc, inc| acc + inc)
        .unwrap();
    let B = (1..2 * n - 1)
        .map(|i| smul2!(s[i], srs_u.0[i].1))
        .reduce(|acc, inc| acc + inc)
        .unwrap();
    let C = (1..2 * n - 1)
        .map(|i| smul1!(s[i], srs_u.0[i - 1].0))
        .reduce(|acc, inc| acc + inc)
        .unwrap();
    let out_c = ct_eq!(pair!(A, H), pair!(G, B)) & 
                ct_eq!(pair!(A, H), pair!(C, srs_u.0[1].1));
    
    // step 8
    let A = (0..n)
        .map(|i| smul1!(s[i], srs_u.0[i].0))
        .reduce(|acc, inc| acc + inc)
        .unwrap();
    let B = (0..n)
        .map(|i| smul1!(s[i], srs_u.1[i].0))
        .reduce(|acc, inc| acc + inc)
        .unwrap();
    let C = (0..n)
        .map(|i| smul1!(s[i], srs_u.1[i].1))
        .reduce(|acc, inc| acc + inc)
        .unwrap();
    let D = (0..n)
        .map(|i| smul2!(s[i], srs_u.1[i].2))
        .reduce(|acc, inc| acc + inc)
        .unwrap();
    let E = (0..n)
        .map(|i| smul2!(s[i], srs_u.1[i].3))
        .reduce(|acc, inc| acc + inc)
        .unwrap();
    let out_d = ct_eq!(pair!(B, H), pair!(G, D)) &
                ct_eq!(pair!(B, H), pair!(A, srs_u.1[0].2)) &
                ct_eq!(pair!(C, H), pair!(G, E)) &
                ct_eq!(pair!(C, H), pair!(A, srs_u.1[0].3));

    // step 9
    let out_e = srs.check_s(&qap).unwrap_or(false);

    // step 10-12
    let out_f = batch.verify(&srs, &s, Phase::TWO).unwrap_or(false);

    // step 13
    let A = (0..m - l)
        .map(|i| smul1!(s[i], srs_s.2[i]))
        .reduce(|acc, inc| acc + inc)
        .unwrap();
    let B = (0..m - l)
        .map(|i| {
            let sum = (0..n)
                .map(|j| add1!(
                    smul1!(u[i].coeff(j), srs_u.1[j].1),
                    smul1!(v[i].coeff(j), srs_u.1[j].0),
                    smul1!(w[i].coeff(j), srs_u.0[j].0)
                ))
                .reduce(|acc, inc| add1!(acc, inc))
                .unwrap();
            smul1!(s[i], sum)
        })
        .reduce(|acc, inc| acc + inc)
        .unwrap();
    let out_g = ct_eq!(pair!(A, srs_s.1), pair!(B, H));

    // step 14
    let Gt = (0..n - 1)
        .into_par_iter()
        .map(|j| smul1!(t.coeff(j), srs_u.0[j].0))
        .reduce(|| zeroG1!(), |acc, inc| add1!(acc, inc));
    let A = (0..n - 1)
        .map(|i| smul1!(s[i], srs_s.3[i]))
        .reduce(|acc, inc| acc + inc)
        .unwrap();
    let B = (0..n - 1)
        .map(|i| smul2!(s[i], srs_u.0[i].1))
        .reduce(|acc, inc| acc + inc)
        .unwrap();
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
