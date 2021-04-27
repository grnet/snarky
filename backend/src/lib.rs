mod elements;
mod operations;

// Export here type aliases to be uniformly 
// used accross the project
pub type Scalar = ::bls12_381::Scalar;
pub type G1Elem = ::bls12_381::G1Affine;
pub type G2Elem = ::bls12_381::G2Affine;


pub trait Backend {

    type Scalar;
    type G1Elem;
    type G2Elem;
    type Bytes1;
    type Bytes2;
    type Gt;

    fn scalar(val: u64) -> Self::Scalar;

    fn zero() -> Self::Scalar;

    fn one() -> Self::Scalar;

    fn rscalar(rng: &mut ::rand::RngCore) -> Self::Scalar;

    fn inv(elem: &Self::Scalar) -> Self::Scalar;

    fn pow(base: &Self::Scalar, exp: usize) -> Self::Scalar;

    fn genG1() -> Self::G1Elem;

    fn genG2() -> Self::G2Elem;

    fn zeroG1() -> Self::G1Elem;

    fn zeroG2() -> Self::G2Elem;

    fn contained_in_G1(elem: &Self::G1Elem) -> bool;

    fn contained_in_G2(elem: &Self::G2Elem) -> bool;

    fn bytes1(elem: &Self::G1Elem) -> Self::Bytes1;

    fn bytes2(elem: &Self::G2Elem) -> Self::Bytes2;

    fn ct_eq<T: subtle::ConstantTimeEq>(elem1: &T, elem2: &T) -> bool {
        // https://docs.rs/subtle/2.4.0/src/subtle/lib.rs.html#67
        bool::from(elem1.ct_eq(elem2))
    }

    fn ct_ne<T: subtle::ConstantTimeEq>(elem1: &T, elem2: &T) -> bool {
        // https://docs.rs/subtle/2.4.0/src/subtle/lib.rs.html#67
        !bool::from(elem1.ct_eq(elem2))
    }

    fn add1(elems: &[G1Elem]) -> G1Elem;

    fn add2(elems: &[G2Elem]) -> G2Elem;

    fn smul1(factor: &Self::Scalar, elem: &Self::G1Elem) -> Self::G1Elem;

    fn smul2(factor: &Self::Scalar, elem: &Self::G2Elem) -> Self::G2Elem;

    fn pair(left: &G1Elem, right: &G2Elem) -> Self::Gt;

    fn hashG1(bytes: &[u8]) -> Self::G1Elem;
}


mod rc_bls12_381;
mod ark_bls12_381;
pub use rc_bls12_381::RcBls12_381;


trait Check<T: Backend> {

    fn sample_usage(&self);// -> T::Scalar;
    fn other_usage(&self) -> T::Scalar;
}

pub struct RcCheck;

impl Check<RcBls12_381> for RcCheck {
    // type T = RcBls12_381;
    // type A = u8;

    // fn sample_usage(&self) -> <RcBls12_381 as Trait>::Scalar {
    fn sample_usage(&self) {
        let zero = RcBls12_381::zero();
        println!("{:?}", zero);
    }

    fn other_usage(&self) -> bls12_381::Scalar {
        let zero = RcBls12_381::zero();
        println!("{:?}", zero);
        zero
    }
}
