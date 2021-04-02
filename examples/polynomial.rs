use snarky::{Univariate, scalar};

fn main() {
    let poly = Univariate::create_from_u64(&vec![1, 2, 3]);
    let r = poly.evaluate(&scalar!(7)).unwrap();
}
