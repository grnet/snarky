use ark_std::rand::SeedableRng;

pub fn snarky_rng() -> ::ark_std::rand::prelude::StdRng {
    let seed: [u8; 32] = ::rand::random();  
    let mut rng = ::ark_std::rand::rngs::StdRng::from_seed(seed);
    rng
}
