use std::convert::TryInto;  // Must be in scope for hashG1
use subtle::ConstantTimeEq; // Must be in scope for ct comparisons
use sha2::Digest;           // Must be in scope for hashG1

use backend::*;
use crate::srs::SRS;
use crate::flow::Phase;

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
    {
        let (G, H) = ctx;
        match 
            ct_eq!(pair!(c.0, H), pair!(G, c.1)) &&
            ct_eq!(pair!(prf, H), pair!(Self::rndoracle(&c), c.1))
        {
            false   => Err(ProofError::DlogFailure),
            _       => Ok(true)
        }
    }
}


// PoKs for scalars used in SRS update
#[derive(Clone, Debug, PartialEq)]
pub struct RhoProof {
    pub aux: G1, 
    pub com: Commitment,
    pub prf: G1,
}

impl RhoProof {
    
    pub fn create(ctx: (&G1, &G2), base: &G1, w: &Scalar) -> Self {
        let (G, H) = ctx;
        let aux = smul1!(w, base);
        let com = (smul1!(w, G), smul2!(w, H));
        Self { 
            aux, 
            com, 
            prf: Dlog::prove(&com, *w),
        }
    }

    pub fn verify(&self, ctx: (&G1, &G2), rho: Option<&Self>) -> Result<bool, ProofError> {
        let (G, H) = ctx;

        let out1 = Dlog::verify(ctx, &self.com, &self.prf).unwrap_or(false);
        let out2 = match rho {
            Some(rho) => {
                ct_eq!(pair!(self.aux, H), pair!(rho.aux, self.com.1))
            },
            None => true,
        };

        match out1 && out2 {
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
#[derive(Clone, Debug, PartialEq)]
pub enum UpdateProof {
    ONE(RhoProof, RhoProof, RhoProof),
    TWO(RhoProof),
}

#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
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

    pub fn verify(&self, srs: &SRS, phase: Phase) -> Result<bool, ProofError> {
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
                            inner = inner && match &batch_u[i][j].verify((&G, &H), match i {
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
                        ct_eq!(srs_u.0[1].0, batch_u[len - 1][2].aux) &&
                        ct_eq!(srs_u.1[0].0, batch_u[len - 1][0].aux) &&
                        ct_eq!(srs_u.1[0].1, batch_u[len - 1][1].aux) &&
                        ct_ne!(batch_u[len - 1][2].aux, zero) &&
                        ct_ne!(batch_u[len - 1][0].aux, zero) &&
                        ct_ne!(batch_u[len - 1][1].aux, zero)
                    }
                }; 
                
                match out1 && out2 {
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
                        acc && match &batch_s[i].verify((&G, &H), match i {
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
                    ct_eq!(pair!(srs_s.0, H), pair!(G, srs_s.1)) &&
                    {
                        let len = batch_s.len();
                        match len > 0 {
                            false   => true,
                            true    => {
                                ct_eq!(srs_s.0, batch_s[len - 1].aux) &&
                                ct_ne!(batch_s[len - 1].aux, zero)
                            }
                        }
                    }
                };

                match out1 && out2 {
                    false   => return Err(ProofError::BatchFailure),
                    _       => Ok(true)
                }
            }
        }
    }
}
