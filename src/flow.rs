// use ark_ec::models::bn::{g1, g2};

use ark_ec::{
    models::{ModelParameters, SWModelParameters},
    short_weierstrass_jacobian::*,
};

use crate::context::Sample;

pub struct QAP {}
pub struct Trapdoor {}

// type UComponent = (Vec<(u32, u64)>, Vec<(u32, u32, u64, u64)>);
// type VComponent = (u32, u64, Vec<u32>, Vec<u32>);
type UComponent = u8;   // TODO: Substitute correct type from ark-ec
type VComponent = i8;   // TODO: Substitute correct type from ark-ec

pub struct SRS {
    u: UComponent,
    v: VComponent,
}

impl SRS {
    fn generate(trapdoor: &Trapdoor, qap: &QAP) -> Self {
        Self {
            u: Self::generate_u(&trapdoor, &qap),
            v: Self::generate_v(&trapdoor, &qap),
        }
    }

    fn generate_u(trapdoor: &Trapdoor, qap: &QAP) -> UComponent {
        0
    }

    fn generate_v(trapdoor: &Trapdoor, qap: &QAP) -> VComponent {
        1
    }
}

pub fn setup(trapdoor: &Trapdoor, qap: &QAP) -> SRS {
    SRS::generate(&trapdoor, &qap)
}

pub fn update(qap: &QAP, srs: &SRS) -> SRS {
    SRS {
        u: 0, 
        v: 1
    }
}

pub fn verify(qap: &QAP, srs: &SRS) -> bool {
    true
}
