use polynomials::Univariate;
use util::SnarkyError;

#[derive(Debug, PartialEq)]
pub struct QAP {
    pub m: usize,
    pub n: usize,
    pub l: usize,
    pub u: Vec<Univariate>,
    pub v: Vec<Univariate>,
    pub w: Vec<Univariate>,
    pub t: Univariate,
}

impl QAP {

    pub fn create(u: Vec<Univariate>, v: Vec<Univariate>, w: Vec<Univariate>, 
        t: Univariate, l: usize) -> Result<Self, SnarkyError> {
        let m = u.len() - 1;
        if v.len() != m + 1 || w.len() != m + 1 {
            let line = line!() - 1;
            return Err(SnarkyError::create("Could not create QAP",
                "Unequal lengths for u, v, w", 
                file!(), 
                line, 
                101
            ))
        } else if l + 1 > m {
            let line = line!() - 1;
            return Err(SnarkyError::create("Could not create QAP", 
                "l is not < m", 
                file!(), 
                line, 
                102
            ))
        } else {
            let n = t.degree() as usize;
            for p in [&u, &v, &w].iter() {
                for i in 0..m + 1 {
                    if p[i].degree() as usize != n - 1 {
                        let line = line!() - 1;
                        return Err(SnarkyError::create("Could not create QAP", 
                            "Detected degree unequal to n-1",
                            file!(), 
                            line,
                            103,
                        ))
                    }
                }
            }
            Ok(Self { m, n, l, u, v, w, t })
        }
    }

    pub fn create_default(m: usize, n: usize, l: usize) -> Result<Self, SnarkyError> {

        let mut coeffs1 = vec![1];
        coeffs1.append(&mut vec![0; n - 1]); // [1] + (n - 1) * [0]
        let u = vec![Univariate::create_from_u64(&coeffs1); m + 1];
        let v = vec![Univariate::create_from_u64(&coeffs1); m + 1];
        let w = vec![Univariate::create_from_u64(&coeffs1); m + 1];

        let mut coeffs2 = vec![1];
        coeffs2.append(&mut vec![0; n]);        // [1] + n * [0]
        let t = Univariate::create_from_u64(&coeffs2);

        Self::create(u, v, w, t, l)
    }

    pub fn dimensions(&self) -> (usize, usize, usize) {
        let m = self.m;
        let n = self.n;
        let l = self.l;
        (m, n, l)
    }

    pub fn collections(&self) -> 
        (&Vec<Univariate>, &Vec<Univariate>, &Vec<Univariate>, &Univariate) 
    {
        let u = &self.u;
        let v = &self.v;
        let w = &self.w;
        let t = &self.t;
        (u, v, w, t)
    }
}
