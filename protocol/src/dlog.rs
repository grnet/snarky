use backend::{scalar, G1_gen, G2_gen, mult_1, mult_2, pair, contained_in_group, 
    bytes_1, bytes_2, hashG1};
use backend::{
    G1Elem as G1,
    G2Elem as G2,
    Scalar,
};

// Must both be in scope for hashG1
use sha2::Digest;
use std::convert::TryInto;

pub fn rndoracle(phi: (G1, G2)) -> G1 {
    hashG1!(&[bytes_1!(phi.0), bytes_2!(phi.1)].concat())
}

pub fn prove_dlog(phi: (G1, G2), witness: Scalar) -> G1 {
    mult_1!(rndoracle(phi), witness)
}

pub fn verify_dlog(G: &G1, H: &G2, phi: (G1, G2), proof: G1) -> bool {
    pair!(phi.0, H) == pair!(G, phi.1) && 
    pair!(proof, H) == pair!(rndoracle(phi), phi.1)
}
