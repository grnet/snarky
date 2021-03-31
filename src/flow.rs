use ark_poly::univariate::DensePolynomial as Polynomial;    // TODO: Maybe use sparse?

type Scalar = u64;  // TODO: Alias correct type from ark-ec
type G1 = u32;      // TODO: Alias correct type from ark-ec
type G2 = u64;      // TODO: Alias correct type from ark-ec
type U = (Vec<(G1, G2)>, Vec<(G1, G1, G2, G2)>);
type V = (G1, G2, Vec<G1>, Vec<G1>);

pub struct QAP {
    l: usize,
    m: usize,
    n: usize,
    // u: Polynomial<...>,
    // v: Polynomial<...>,
    // w: Polynomial<...>,
    // t: Polynomial<...>,
}

impl QAP {
    pub fn create_default(m: usize, n: usize, l: usize) -> Self {
        Self { l, m, n }
    }
}

pub struct Trapdoor {
    a: Scalar,
    b: Scalar,
    d: Scalar,
    x: Scalar,
}

impl Trapdoor {
    fn create(a: Scalar, b: Scalar, d: Scalar, x: Scalar) -> Self {
        Self { a, b, d, x }
    }
    pub fn create_from_units() -> Self {
        Self::create(1, 1, 1, 1)
    }
    // fn create_from_random() -> Self {
    //     Self {}
    // }
}

pub struct SRS {
    u: U,
    v: V,
}

impl SRS {
    fn generate(trapdoor: &Trapdoor, qap: &QAP) -> Self {
        Self {
            u: Self::generate_u(&trapdoor, &qap),
            v: Self::generate_v(&trapdoor, &qap),
        }
    }

    fn generate_u(trapdoor: &Trapdoor, qap: &QAP) -> U {
        (Vec::<(G1, G2)>::new(), Vec::<(G1, G1, G2, G2)>::new())
    }

    fn generate_v(trapdoor: &Trapdoor, qap: &QAP) -> V {
        (0, 0, Vec::<G1>::new(), Vec::<G1>::new())
    }
}

pub fn setup(trapdoor: &Trapdoor, qap: &QAP) -> SRS {
    SRS::generate(&trapdoor, &qap)
}

pub fn update(qap: &QAP, srs: &SRS) -> SRS {
    SRS {
        u: (Vec::<(G1, G2)>::new(), Vec::<(G1, G1, G2, G2)>::new()),
        v: (0, 0, Vec::<G1>::new(), Vec::<G1>::new()),
    }
}

pub fn verify(qap: &QAP, srs: &SRS) -> bool {
    true
}
