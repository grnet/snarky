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

    // step 3-4
    let out_b = batch.verify(&srs, Phase::ONE).unwrap_or(false);

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
    let out_f = batch.verify(&srs, Phase::TWO).unwrap_or(false);

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