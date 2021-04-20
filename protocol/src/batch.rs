use backend::{Scalar, zeroG1, genG1, genG2, pair};
use crate::prover::RhoProof;
use crate::flow::Phase;
use crate::flow::SRS;


#[derive(Clone, Debug, PartialEq)]
pub enum Witness {
    ONE(Scalar, Scalar, Scalar),
    TWO(Scalar),
}


// Collections of PoKs produced during SRS update,
// corresponding to respective phases (ONE or TWO)
// Note: No verify functionality is possible at this
// level since each update-proof is verified against
// its previous one in the containing batch.
#[derive(Clone, Debug, PartialEq)]
pub enum UpdateProof {
    ONE(RhoProof, RhoProof, RhoProof),
    TWO(RhoProof),
}

impl UpdateProof {
    pub fn create(srs: &SRS, w: &Witness) -> Self{
        let (G, H) = (genG1!(), genG2!());
        match w {
            Witness::ONE(a, b, x) => {
                // phase 1, step 3-6
                let srs_u = &srs.u;
                UpdateProof::ONE(
                    RhoProof::for_value((&G, &H, srs_u.1[0].0), &a),
                    RhoProof::for_value((&G, &H, srs_u.1[0].1), &b),
                    RhoProof::for_value((&G, &H, srs_u.0[1].0), &x),
                )
            },
            Witness::TWO(d) => {
                // phase 2, step 3-4
                let srs_s = &srs.s;
                UpdateProof::TWO(
                    RhoProof::for_value((&G, &H, srs_s.0), &d)
                )
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BatchProof {
    pub phase_1: Vec<[RhoProof; 3]>,
    pub phase_2: Vec<RhoProof>,
}

impl BatchProof {

    pub fn initiate() -> Self {
        Self {
            phase_1: Vec::new(),
            phase_2: Vec::new()
        }
    }

    pub fn append(&mut self, proof: UpdateProof) {
        match proof {
            UpdateProof::ONE(r1, r2, r3) => {
                self.phase_1.push([r1, r2, r3]);
            },
            UpdateProof::TWO(r) => {
                self.phase_2.push(r);
            },
        }
    }

    pub fn verify(&self, srs: &SRS, phase: Phase) -> bool {
        let zero = zeroG1!();
        let G = genG1!();
        let H = genG2!();
        match phase {
            Phase::ONE => {
                let batch_u = &self.phase_1;
                let srs_u = &srs.u;

                // step 3
                for i in 0..batch_u.len() {
                    for j in 0..3 {
                        let rho = &batch_u[i][j];
                        let prf = if i != 0 { Some(&batch_u[i - 1][j]) } else { None };
                        match rho.verify((&G, &H), prf)
                        {
                            true    => continue,
                            _       => return false,
                        }
                    }
                }

                // step 4
                let len = batch_u.len();
                match len > 0 && !(
                    srs_u.0[1].0 == batch_u[len - 1][2].0 &&
                    srs_u.1[0].0 == batch_u[len - 1][0].0 &&
                    srs_u.1[0].1 == batch_u[len - 1][1].0 &&
                    batch_u[len - 1][2].0 != zero &&
                    batch_u[len - 1][0].0 != zero &&
                    batch_u[len - 1][1].0 != zero
                )
                {
                    true    => false,
                    _       => true,
                }
            },
            Phase::TWO => {
                let batch_s = &self.phase_2;
                let srs_s = &srs.s;

                // step 8
                for i in 0..batch_s.len() {
                    let rho = &batch_s[i];
                    let prf = if i != 0 { Some(&batch_s[i - 1]) } else { None };
                    match rho.verify((&G, &H), prf)
                    {
                        true    => continue,
                        _       => return false,
                    }
                }

                // step 9
                match pair!(srs_s.0, H) != pair!(G, srs_s.1) {
                    true    => return false,
                    false   => {
                        let len = batch_s.len();
                        match len > 0 && !(
                            srs_s.0 == batch_s[len - 1].0 &&
                            batch_s[len - 1].0 != zero
                        ) 
                        {
                            true    => false,
                            _       => true,
                        }
                    }
                }
            }
        }
    }
}
