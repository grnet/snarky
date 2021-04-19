mod elements;
mod operations;

// Export here type aliases to be uniformly 
// used accross the project
pub type Scalar = ::bls12_381::Scalar;
pub type G1Elem = ::bls12_381::G1Affine;
pub type G2Elem = ::bls12_381::G2Affine;
