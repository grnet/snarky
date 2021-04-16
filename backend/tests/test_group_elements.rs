use backend::{
    G1_gen, G2_gen, G1_zero, G2_zero, contained_in_group, bytes_1, bytes_2
};
use backend::{scalar, mult_1, mult_2};
use bls12_381::{Scalar, G1Affine, G2Affine};

#[test]
fn test_G1_gen() {
    assert_eq!(
        G1Affine::generator(), 
        G1_gen!()
    );
}

#[test]
fn test_G2_gen() {
    assert_eq!(
        G2Affine::generator(), 
        G2_gen!()
    );
}

#[test]
fn test_G1_zero() {
    assert_eq!(
        G1Affine::from(G1Affine::generator() * Scalar::zero()), 
        G1_zero!()
    );
}

#[test]
fn test_G2_zero() {
    assert_eq!(
        G2Affine::from(G2Affine::generator() * Scalar::zero()), 
        G2_zero!()
    );
}

#[test]
fn test_contained_in_group() {
    let G = G1_gen!();
    let H = G2_gen!();
    let parametrization = [0, 1, 2, 7, 11, 666, 389473847];
    for factor in &parametrization {
        let f = scalar!(*factor);
        let elem1 = mult_1!(G, f);
        let elem2 = mult_2!(H, f);
        assert!(contained_in_group!(elem1));
        assert!(contained_in_group!(elem2));
    }
}

#[test]
fn test_bytes_1() {
    let mut expected: [u8; 96] = [0; 96];
    expected[0] = 64;
    assert!(bytes_1!(G1_zero!()) == expected);
}

#[test]
fn test_bytes_2() {
    let mut expected: [u8; 96] = [0; 96];
    expected[0] = 192;
    assert!(bytes_2!(G2_zero!()) == expected);
}
