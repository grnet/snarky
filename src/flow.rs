use ark_ec::PairingEngine;

pub struct SRS {
}

pub struct QAP {
}

pub fn setup() -> SRS {
    return SRS {}
}

pub fn update(qap: &QAP, srs: &SRS) -> SRS {
    return SRS {}
}

pub fn verify(qap: &QAP, srs: &SRS) -> bool {
    return true
}
