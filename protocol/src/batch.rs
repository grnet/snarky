use subtle::ConstantTimeEq; // Must be in scope for ct equality checks
use backend::*;
use crate::prover::RhoProof;
use crate::flow::Phase;
use crate::flow::SRS;
use crate::prover::ProofError;


#[derive(Clone, Debug, PartialEq)]
pub enum Witness {
    ONE(Scalar, Scalar, Scalar),
    TWO(Scalar),
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
                for i in 0..batch_u.len() {
                    for j in 0..3 {
                        match &batch_u[i][j].verify((&G, &H), match i {
                            0 => None,
                            _ => Some(&batch_u[i - 1][j])
                        })
                        {
                            Ok(true)    => continue,
                            _           => return Err(ProofError::BatchFailure)
                        }
                    }
                }

                // step 4
                let len = batch_u.len();
                match len > 0 && !(
                    ct_eq!(srs_u.0[1].0, batch_u[len - 1][2].aux) &&
                    ct_eq!(srs_u.1[0].0, batch_u[len - 1][0].aux) &&
                    ct_eq!(srs_u.1[0].1, batch_u[len - 1][1].aux) &&
                    ct_ne!(batch_u[len - 1][2].aux, zero) &&
                    ct_ne!(batch_u[len - 1][0].aux, zero) &&
                    ct_ne!(batch_u[len - 1][1].aux, zero)
                )
                {
                    true    => return Err(ProofError::BatchFailure),
                    _       => Ok(true),
                }
            },
            Phase::TWO => {
                let batch_s = &self.batch_2;
                let srs_s = &srs.s;

                // step 8
                for i in 0..batch_s.len() {
                    match &batch_s[i].verify((&G, &H), match i {
                        0 => None,
                        _ => Some(&batch_s[i - 1])
                    })
                    {
                        Ok(true) => continue,
                        _ => return Err(ProofError::BatchFailure)
                    }
                }

                // step 9
                match 
                    ct_ne!(pair!(srs_s.0, H), pair!(G, srs_s.1)) 
                {
                    true    => return Err(ProofError::BatchFailure),
                    false   => {
                        let len = batch_s.len();
                        match len > 0 && 
                        !(
                            ct_eq!(srs_s.0, batch_s[len - 1].aux) &&
                            ct_ne!(batch_s[len - 1].aux, zero)
                        ) 
                        {
                            true    => Err(ProofError::BatchFailure),
                            _       => Ok(true),
                        }
                    }
                }
            }
        }
    }
}
