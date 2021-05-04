use std::rand;
use ark_std::rand::{StdRng, SeedableRng};
use ark_std::rand as ark_rand;

pub fn snarky_rng() -> StdRng {
    let seed: [u8; 32] = rand::random();  
    let mut rng = StdRng::from_seed(seed);
    rng
}
