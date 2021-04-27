use backend::Scalar as F;
use polynomials::Univariate;
use util::SnarkyError;
use backend::Backend;

#[derive(Debug, PartialEq)]
pub struct ExpQAP<T: Backend> {
    pub m: usize,
    pub n: usize,
    pub l: usize,
    pub u: Vec<Univariate<T::Scalar>>,
    pub v: Vec<Univariate<T::Scalar>>,
    pub w: Vec<Univariate<T::Scalar>>,
    pub t: Univariate<T::Scalar>,
}

impl<T: Backend> ExpQAP<T> {
    pub fn create(
        u: Vec<Univariate<T::Scalar>>, 
        v: Vec<Univariate<T::Scalar>>, 
        w: Vec<Univariate<T::Scalar>>, 
        t: Univariate<T::Scalar>, 
        l: usize
    ) -> Result<Self, SnarkyError> {
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

    pub fn create_default(m: usize, n: usize, l: usize) -> Result<Self, SnarkyError> 
        where <T as Backend>::Scalar: From<u64>, <T as Backend>::Scalar: Clone
    {

        let mut coeffs1 = vec![1];
        coeffs1.append(&mut vec![0; n - 1]);    // [1] + (n - 1) * [0]
        let u = vec![Univariate::create_from_u64(&coeffs1); m + 1];
        let v = vec![Univariate::create_from_u64(&coeffs1); m + 1];
        let w = vec![Univariate::create_from_u64(&coeffs1); m + 1];

        let mut coeffs2 = vec![1];
        coeffs2.append(&mut vec![0; n]);        // [1] + n * [0]
        let t = Univariate::create_from_u64(&coeffs2);

        Self::create(u, v, w, t, l)
    }

    pub fn shape(&self) -> (usize, usize, usize) {
        let m = self.m;
        let n = self.n;
        let l = self.l;
        (m, n, l)
    }

    pub fn collections(&self) -> (
        &Vec<Univariate<T::Scalar>>, 
        &Vec<Univariate<T::Scalar>>, 
        &Vec<Univariate<T::Scalar>>, 
        &Univariate<T::Scalar>
    ) 
    {
        let u = &self.u;
        let v = &self.v;
        let w = &self.w;
        let t = &self.t;
        (u, v, w, t)
    }
}

#[derive(Debug, PartialEq)]
pub struct QAP {
    pub m: usize,
    pub n: usize,
    pub l: usize,
    pub u: Vec<Univariate<F>>,
    pub v: Vec<Univariate<F>>,
    pub w: Vec<Univariate<F>>,
    pub t: Univariate<F>,
}

impl QAP {

    pub fn create(
        u: Vec<Univariate<F>>, 
        v: Vec<Univariate<F>>, 
        w: Vec<Univariate<F>>, 
        t: Univariate<F>, 
        l: usize
    ) -> Result<Self, SnarkyError> {
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

    pub fn shape(&self) -> (usize, usize, usize) {
        let m = self.m;
        let n = self.n;
        let l = self.l;
        (m, n, l)
    }

    pub fn collections(&self) -> 
        (&Vec<Univariate<F>>, &Vec<Univariate<F>>, &Vec<Univariate<F>>, &Univariate<F>) 
    {
        let u = &self.u;
        let v = &self.v;
        let w = &self.w;
        let t = &self.t;
        (u, v, w, t)
    }
}
