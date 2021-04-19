use backend::{scalar, genG1, genG2, smul1, smul2, pair, contained_in_group, 
    bytes1, bytes2, hashG1};
use backend::{
    G1Elem as G1,
    G2Elem as G2,
    Scalar,
};

// Must both be in scope for hashG1
use sha2::Digest;
use std::convert::TryInto;

pub fn rndoracle(phi: (G1, G2)) -> G1 {
    hashG1!(&[bytes1!(phi.0), bytes2!(phi.1)].concat())
}

pub fn prove_dlog(phi: (G1, G2), witness: Scalar) -> G1 {
    smul1!(rndoracle(phi), witness)
}

pub fn verify_dlog(G: &G1, H: &G2, phi: (G1, G2), proof: G1) -> bool {
    pair!(phi.0, H) == pair!(G, phi.1) && 
    pair!(proof, H) == pair!(rndoracle(phi), phi.1)
}
