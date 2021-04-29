use rand::RngCore;          // Must be in scope for update
use subtle::ConstantTimeEq; // Must be in scope for ct comparisons

use backend::*;
use circuits::QAP;
use crate::prover::{RhoProof, Witness, UpdateProof};

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Phase {
    ONE = 1,
    TWO = 2,
}

pub fn update(qap: &QAP, srs: &mut SRS, batch: &mut BatchProof, phase: Phase) {

    // phase 1/2: step 2
    let witness = match phase {
        Phase::ONE => {
            Witness::ONE(
                rscalar!(::util::snarky_rng()),
                rscalar!(::util::snarky_rng()),
                rscalar!(::util::snarky_rng()),
            )
        },
        Phase::TWO => {
            Witness::TWO(rscalar!(
                ::util::snarky_rng()
            ))
        }
    };

    // phase 1: steps 3-7; phase 2: steps 3-4
    batch.append(UpdateProof::create(
        &srs, 
        &witness
    ));

    // phase 1: steps 8-10; phase 2: step 5
    srs.update(&qap, witness);
}


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
        .fold(true, |acc, i| {
            acc &
                ct_eq!(pair!(srs_u.0[i].0, H), pair!(G, srs_u.0[i].1)) &
                ct_eq!(pair!(srs_u.0[i].0, H), pair!(srs_u.0[i - 1].0, srs_u.0[1].1))
        });
    
    // step 6
    let out_d = (0..n) 
        .fold(true, |acc, i| {
            acc &
                ct_eq!(pair!(srs_u.1[i].0, H), pair!(G, srs_u.1[i].2)) &
                ct_eq!(pair!(srs_u.1[i].0, H), pair!(srs_u.0[i].0, srs_u.1[0].2)) &
                ct_eq!(pair!(srs_u.1[i].1, H), pair!(G, srs_u.1[i].3)) &
                ct_eq!(pair!(srs_u.1[i].1, H), pair!(srs_u.0[i].0, srs_u.1[0].3))
        });
    

    // step 7
    let out_e = srs.check_s(&qap).unwrap_or(false);

    // step 8-9
    let out_f = batch.verify(&srs, Phase::TWO).unwrap_or(false);

    // step 10
    let out_g = (0..m - l)
        .fold(true, |acc, i| {
            let s_i = (0..n)
                .fold(zeroG1!(), |acc, j| {
                    add1!(acc, add1!(
                        smul1!(u[i].coeff(j), srs_u.1[j].1),
                        smul1!(v[i].coeff(j), srs_u.1[j].0),
                        smul1!(w[i].coeff(j), srs_u.0[j].0)
                    ))
                });
            acc & ct_eq!(pair!(srs_s.2[i], srs_s.1), pair!(s_i, H))
        });

    // step 11
    let out_h = {
        let Gt = (0..n - 1)
            .fold(zeroG1!(), |acc, j| {
                add1!(acc, smul1!(t.coeff(j), srs_u.0[j].0))
            });
        (0..n - 1)
            .fold(true, |acc, i| {
                acc & ct_eq!(pair!(srs_s.3[i], srs_s.1), pair!(Gt, srs_u.0[i].1))
            })
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
