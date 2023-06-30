use super::*;
use std::f64::consts;

#[test]
fn eulers_number_test() {
    let eulers = calculate("e");
    assert!(
        eulers == 1_f64.exp(),
        "Euler's Assertion Failed: e^1 is not equal to e ({eulers})",
    );
}

#[test]
fn pi_test() {
    let pi = calculate("pi");
    assert!(
        pi == consts::PI,
        "Pi Assertion Failed: Pi was calculated as {pi}"
    );
}
