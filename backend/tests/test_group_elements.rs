use backend::{
    genG1, genG2, zeroG1, zeroG2, contained_in_group, bytes1, bytes2
};
use backend::{scalar, smul1, smul2};
use bls12_381::{Scalar, G1Affine, G2Affine};

#[test]
fn test_genG1() {
    assert_eq!(
        G1Affine::generator(), 
        genG1!()
    );
}

#[test]
fn test_genG2() {
    assert_eq!(
        G2Affine::generator(), 
        genG2!()
    );
}

#[test]
fn test_zeroG1() {
    assert_eq!(
        G1Affine::from(G1Affine::generator() * Scalar::zero()), 
        zeroG1!()
    );
}

#[test]
fn test_zeroG2() {
    assert_eq!(
        G2Affine::from(G2Affine::generator() * Scalar::zero()), 
        zeroG2!()
    );
}

#[test]
fn test_contained_in_group() {
    let G = genG1!();
    let H = genG2!();
    let parametrization = [0, 1, 2, 7, 11, 666, 389473847];
    for factor in &parametrization {
        let f = scalar!(*factor);
        let elem1 = smul1!(G, f);
        let elem2 = smul2!(H, f);
        assert!(contained_in_group!(elem1));
        assert!(contained_in_group!(elem2));
    }
}

#[test]
fn test_bytes1() {
    let mut expected: [u8; 96] = [0; 96];
    expected[0] = 64;
    assert!(bytes1!(zeroG1!()) == expected);
}

#[test]
fn test_bytes2() {
    let mut expected: [u8; 96] = [0; 96];
    expected[0] = 192;
    assert!(bytes2!(zeroG2!()) == expected);
}
