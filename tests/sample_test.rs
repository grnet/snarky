//! Contains integration tests involving `snarky` public functions.
//! Common test setup taken from the `common` module.

// use snarky;
mod common;

#[test]
fn test_test() {
    common::setup();
    assert_eq!(0, 0);
}
