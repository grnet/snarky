use subtle::ConstantTimeEq; // Must be in scope for ct equality checks
use sha2::Digest;           // Must be in scope for hashG1
use std::convert::TryInto;  // Must be in scope for hashG1
use backend::*;

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
        match Dlog::verify(ctx, &self.com, &self.prf) {
            Ok(true) => {
                match rho {
                    Some(rho) => {
                        match 
                            ct_eq!(pair!(self.aux, H), pair!(rho.aux, self.com.1))
                        {
                            false   => Err(ProofError::RhoFailure),
                            _       => Ok(true)
                        }
                    },
                    None => Ok(true),
                }
            },
            _ => Err(ProofError::RhoFailure)
        }
    }
}
