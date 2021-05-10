use subtle::ConstantTimeEq; // Must be in scope for ct comparisons

use sha2::Digest;
use std::convert::TryInto;
use ark_ff::FromBytes;
use std::io::Cursor;

use backend::*;
use crate::srs::SRS;
use crate::updater::Phase;

use ark_ec::AffineCurve;            // Needed for group inclusion check
use ark_ec::PairingEngine;          // Needed for pairing
use num_traits::identities::Zero;   // Needed for zero constructions
use num_traits::identities::One;    // Needed for one constructions
use ark_ff::fields::Field;          // Needed for pow
use ark_ff::ToBytes;
use ark_std::rand::Rng as ArkRng;   // Must be in scope for rscalar
use ark_bls12_381;

use rayon::prelude::*;

type G1 = G1Elem;
type G2 = G2Elem;


// Indicates proof-verification failure
#[derive(Debug, PartialEq)]
pub enum ProofError {
    DlogFailure,
    RhoFailure,
    BatchFailure,
}


pub struct Dlog;
type Commitment = (G1, G2);

impl Dlog {

    pub fn rndoracle(c: &Commitment) -> G1 {
        let bytes = [bytes1!(c.0), bytes2!(c.1)].concat();
        hashG1!(&bytes)
    }
    
    pub fn prove(c: &Commitment, witness: Scalar) -> G1 {
        smul1!(witness, Self::rndoracle(&c))
    }

    pub fn verify(ctx: (&G1, &G2), c: &Commitment, prf: &G1) 
        -> Result<bool, ProofError> 
    // TODO: ARK: Simplify signature
    {
        let (G, H) = ctx;
        match 
            ct_eq!(pair!(c.0, *H), pair!(*G, c.1)) &
            ct_eq!(pair!(*prf, *H), pair!(Self::rndoracle(&c), c.1))
        {
            false   => Err(ProofError::DlogFailure),
            _       => Ok(true)
        }
    }
}


// PoKs for scalars used in SRS update
#[derive(Debug, PartialEq)]
pub struct RhoProof {
    pub aux: G1, 
    pub com: Commitment,
    pub prf: G1,
}

impl RhoProof {
    
    pub fn create(ctx: (&G1, &G2), base: &G1, w: &Scalar) -> Self {
        // TODO: ARK: Simplify signatures
        let (G, H) = ctx;
        let aux = smul1!(*w, *base);
        let com = (smul1!(*w, *G), smul2!(*w, *H));
        Self { 
            aux, 
            com, 
            prf: Dlog::prove(&com, *w),
        }
    }

    pub fn verify(&self, ctx: (&G1, &G2), rho: Option<&Self>) -> Result<bool, ProofError> {
        // TODO: ARK: Simplify signatures
        let (G, H) = ctx;

        let out1 = Dlog::verify(ctx, &self.com, &self.prf).unwrap_or(false);
        let out2 = match rho {
            Some(rho) => {
                ct_eq!(pair!(self.aux, *H), pair!(rho.aux, self.com.1))
            },
            None => true,
        };

        match out1 & out2 {
            false   => Err(ProofError::RhoFailure),
            _       => Ok(true)
        }
    }
}


// Collections of PoKs produced during SRS update,
// corresponding to respective phases (ONE or TWO)
// Note: No verify functionality is possible at this
// level since each update-proof is verified against
// its previous one in the containing batch.
#[derive(Debug, PartialEq)]
pub enum UpdateProof {
    ONE(RhoProof, RhoProof, RhoProof),
    TWO(RhoProof),
}

#[derive(Debug, PartialEq)]
pub enum Witness {
    ONE(Scalar, Scalar, Scalar),
    TWO(Scalar),
}

impl UpdateProof {
    pub fn create(srs: &SRS, w: &Witness) -> Self{
        let (G, H) = (genG1!(), genG2!());
        match w {
            Witness::ONE(a, b, x) => {
                // phase 1, step 3-6
                let srs_u = &srs.u;
                UpdateProof::ONE(
                    RhoProof::create((&G, &H), &srs_u.1[0].0, &a),
                    RhoProof::create((&G, &H), &srs_u.1[0].1, &b),
                    RhoProof::create((&G, &H), &srs_u.0[1].0, &x),
                )
            },
            Witness::TWO(d) => {
                // phase 2, step 3-4
                let srs_s = &srs.s;
                UpdateProof::TWO(
                    RhoProof::create((&G, &H), &srs_s.0, &d)
                )
            },
        }
    }
}


// Batch of update-proofs
#[derive(Debug, PartialEq)]
pub struct BatchProof {
    pub batch_1: Vec<[RhoProof; 3]>,    // Contains phase 1 update-proofs
    pub batch_2: Vec<RhoProof>,         // Contains phase 2 update-proofs
}

impl BatchProof {

    pub fn initiate() -> Self {
        Self {
            batch_1: Vec::new(),
            batch_2: Vec::new()
        }
    }

    pub fn append(&mut self, proof: UpdateProof) {
        match proof {
            UpdateProof::ONE(r1, r2, r3) => {
                self.batch_1.push([r1, r2, r3]);
            },
            UpdateProof::TWO(r) => {
                self.batch_2.push(r);
            },
        }
    }

    pub fn verify_naive(&self, srs: &SRS, phase: Phase) -> Result<bool, ProofError> {
        let (G, H) = (genG1!(), genG2!());
        let zero = zeroG1!();
        match phase {
            Phase::ONE => {
                let batch_u = &self.batch_1;
                let srs_u = &srs.u;

                // step 3
                let out1 = (0..batch_u.len())
                    .fold(true, |acc, i| { 
                        let mut inner = true;
                        for j in 0..3 {
                            inner = inner & match &batch_u[i][j].verify((&G, &H), match i {
                                0 => None,
                                _ => Some(&batch_u[i - 1][j])
                            })
                            {
                                Err(ProofError::RhoFailure) => false,
                                _ => true
                            };
                        }
                        acc && inner
                    });

                // step 4
                let len = batch_u.len();
                let out2 = match len > 0 {
                    false   => true,
                    true    => {
                        ct_eq!(srs_u.0[1].0, batch_u[len - 1][2].aux) &
                        ct_eq!(srs_u.1[0].0, batch_u[len - 1][0].aux) &
                        ct_eq!(srs_u.1[0].1, batch_u[len - 1][1].aux) &
                        ct_ne!(batch_u[len - 1][2].aux, zero) &
                        ct_ne!(batch_u[len - 1][0].aux, zero) &
                        ct_ne!(batch_u[len - 1][1].aux, zero)
                    }
                }; 
                
                match out1 & out2 {
                    false   => Err(ProofError::BatchFailure),
                    _       => Ok(true)
                }
            },
            Phase::TWO => {
                let batch_s = &self.batch_2;
                let srs_s = &srs.s;

                // step 8
                let out1 = (0..batch_s.len()) 
                    .fold(true, |acc, i| {
                        acc & match &batch_s[i].verify((&G, &H), match i {
                            0 => None,
                            _ => Some(&batch_s[i - 1])
                        })
                        {
                            Err(ProofError::RhoFailure) => false,
                            _ => true
                        }
                    });
                
                // step 9
                let out2 = {
                    ct_eq!(pair!(srs_s.0, H), pair!(G, srs_s.1)) &
                    {
                        let len = batch_s.len();
                        match len > 0 {
                            false   => true,
                            true    => {
                                ct_eq!(srs_s.0, batch_s[len - 1].aux) &
                                ct_ne!(batch_s[len - 1].aux, zero)
                            }
                        }
                    }
                };

                match out1 & out2 {
                    false   => return Err(ProofError::BatchFailure),
                    _       => Ok(true)
                }
            }
        }
    }

    pub fn verify(&self, srs: &SRS, s: &[backend::Scalar], phase: Phase) -> Result<bool, ProofError> {
        let (G, H) = (genG1!(), genG2!());
        let zero = zeroG1!();
        match phase {
            Phase::ONE => {
                let batch_u = &self.batch_1;
                let srs_u = &srs.u;

                // step 4-5
                let mut out1 = true;
                for j in 0..3 {
                    if batch_u.len() > 2 {
                        let (A, B) = (2..batch_u.len())
                            .into_par_iter()
                            .map(|i| {                                          // 4
                                let rho      = &batch_u[i][j];
                                let rho_prev = &batch_u[i - 1][j];
                                (
                                    smul1!(s[i], rho.aux),
                                    pair!(smul1!(s[i], rho_prev.aux), rho.com.1)
                                )
                            })
                            .reduce(|| (zeroG1!(), unit!()), 
                                |
                                    (a1, b1),
                                    (a2, b2),
                                |
                                (
                                    a1 + a2,
                                    b1 * b2,
                                )
                            );
                        out1 = out1 & ct_eq!(pair!(A, H), B);                   // 5.(a)
                    }
                    if batch_u.len() > 1 {
                        let (C, D, E, F) = (1..batch_u.len())
                            .into_par_iter()
                            .map(|i| {
                                let rho      = &batch_u[i][j];                  // 4
                                let rho_prev = &batch_u[i - 1][j];
                                let R = Dlog::rndoracle(&rho.com);
                                (
                                    smul1!(s[i], rho.com.0),
                                    smul2!(s[i], rho.com.1),
                                    smul1!(s[i], rho.prf),
                                    pair!(smul1!(s[i], R), rho.com.1),
                                )
                            })
                            .reduce(|| (zeroG1!(), zeroG2!(), zeroG1!(), unit!()), 
                                |
                                    (a1, b1, c1, d1),
                                    (a2, b2, c2, d2),
                                |
                                (
                                    a1 + a2,
                                    b1 + b2,
                                    c1 + c2,
                                    d1 * d2,
                                )
                            );
                        out1 = out1 & ct_eq!(pair!(C, H), pair!(G, D));         // 5.(b)
                                    & ct_eq!(pair!(E, H), F);                   // 5.(c)
                    }
                }

                // step 6
                let len = batch_u.len();
                let out2 = match len > 0 {
                    false   => true,
                    true    => {
                        ct_eq!(srs_u.0[1].0, batch_u[len - 1][2].aux) &
                        ct_eq!(srs_u.1[0].0, batch_u[len - 1][0].aux) &
                        ct_eq!(srs_u.1[0].1, batch_u[len - 1][1].aux) &
                        ct_ne!(batch_u[len - 1][2].aux, zero) &
                        ct_ne!(batch_u[len - 1][0].aux, zero) &
                        ct_ne!(batch_u[len - 1][1].aux, zero)
                    }
                }; 
                
                match out1 & out2 {
                    false   => Err(ProofError::BatchFailure),
                    _       => Ok(true)
                }
            },
            Phase::TWO => {
                let batch_s = &self.batch_2;
                let srs_s = &srs.s;

                // step 10-11
                let mut out1 = true;
                if batch_s.len() > 2 {
                    let (A, B) = (2..batch_s.len())
                        .into_par_iter()
                        .map(|i| {                                          // 10
                            let rho      = &batch_s[i];
                            let rho_prev = &batch_s[i - 1];
                            (
                                smul1!(s[i], rho.aux),
                                pair!(smul1!(s[i], rho_prev.aux), rho.com.1)
                            )
                        })
                        .reduce(|| (zeroG1!(), unit!()), 
                            |
                                (a1, b1),
                                (a2, b2),
                            |
                            (
                                a1 + a2,
                                b1 * b2,
                            )
                        );
                    out1 = out1 & ct_eq!(pair!(A, H), B);                   // 11.(a)
                }
                if batch_s.len() > 1 {
                    let (C, D, E, F) = (1..batch_s.len())
                        .into_par_iter()
                        .map(|i| {
                            let rho      = &batch_s[i];                     // 10
                            let rho_prev = &batch_s[i - 1];
                            let R = Dlog::rndoracle(&rho.com);
                            (
                                smul1!(s[i], rho.com.0),
                                smul2!(s[i], rho.com.1),
                                smul1!(s[i], rho.prf),
                                pair!(smul1!(s[i], R), rho.com.1),
                            )
                        })
                        .reduce(|| (zeroG1!(), zeroG2!(), zeroG1!(), unit!()), 
                            |
                                (a1, b1, c1, d1),
                                (a2, b2, c2, d2),
                            |
                            (
                                a1 + a2,
                                b1 + b2,
                                c1 + c2,
                                d1 * d2,
                            )
                        );
                    out1 = out1 & ct_eq!(pair!(C, H), pair!(G, D));         // 11.(b)
                                & ct_eq!(pair!(E, H), F);                   // 11.(c)
                }
 
                // step 12
                let out2 = {
                    ct_eq!(pair!(srs_s.0, H), pair!(G, srs_s.1)) &
                    {
                        let len = batch_s.len();
                        match len > 0 {
                            false   => true,
                            true    => {
                                ct_eq!(srs_s.0, batch_s[len - 1].aux) &
                                ct_ne!(batch_s[len - 1].aux, zero)
                            }
                        }
                    }
                };

                match out1 & out2 {
                    false   => return Err(ProofError::BatchFailure),
                    _       => Ok(true)
                }
            }
        }
    }
}
