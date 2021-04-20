use backend::{
    Scalar,
    G1Elem as G1,
    G2Elem as G2,
    scalar,
    genG1,
    genG2,
    smul1,
    smul2,
    pair,
    contained_in_group,
    bytes1,
    bytes2,
    hashG1,
};

// Must both be in scope for hashG1
use sha2::Digest;
use std::convert::TryInto;

pub fn rndoracle(phi: (G1, G2)) -> G1 {
    hashG1!(&[bytes1!(phi.0), bytes2!(phi.1)].concat())
}

pub fn prove_dlog(phi: (G1, G2), witness: Scalar) -> G1 {
    smul1!(witness, rndoracle(phi))
}

pub fn verify_dlog(G: &G1, H: &G2, phi: (G1, G2), proof: G1) -> bool {
    pair!(phi.0, H) == pair!(G, phi.1) &&
    pair!(proof, H) == pair!(rndoracle(phi), phi.1)
}

// PoK for the value used in SRS update
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateProof(
    pub G1, 
    pub G1, 
    pub G2, 
    pub G1
);

impl UpdateProof {
    
    pub fn for_value(ctx: (&G1, &G2, G1), val: &Scalar) -> Self {
        let (G, H, base) = ctx;
        let prf = prove_dlog((smul1!(val, G), smul2!(val, H)), *val);
        Self(
            smul1!(val, base),
            smul1!(val, G),
            smul2!(val, H),
            prf
        )
    }

    pub fn verify(&self, ctx: (&G1, &G2), prf: Option<&Self>) -> bool {
        let (G, H) = ctx;
        match verify_dlog(&G, &H, (self.1, self.2), self.3) {
            true => {
                match prf {
                    Some(prf) => {
                        pair!(self.0, H) == pair!(prf.0, self.2)
                    },
                    None => true,
                }
            }
            _ => false
        }
    }
}
