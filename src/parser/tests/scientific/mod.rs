use super::*;

mod trig;

#[test]
fn scientific_test() {
    let logarithm = calculate("log10");
    assert_eq!(
        logarithm, 1_f64,
        "Logarithm_10 Assertion failed: log10 became {logarithm}"
    );

    let natural_log = calculate("ln10");
    assert_eq!(
        natural_log,
        10_f64.ln(),
        "Natural Logarithm Assertion failed: ln10 became {natural_log}"
    );
}

#[test]
fn factorial_test() {
    let fact = calculate("10!") as i128;
    assert_eq!(
        fact,
        (10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2),
        "Factorial Assertion failed: 10! became {fact}"
    );
}

#[test]
fn combination_test() {
    let combination = calculate("-(2*3)+7/5^2-log10");
    assert_eq!(
        combination, -6.72,
        "Combination Assertion failed: -(2*3) + 7/(5^2) - log10 became {combination}"
    );

    let other_combination = calculate("-(2*3)+(7/5)^2-log10");
    assert_eq!(
        other_combination, -5.04,
        "Combination Assertion failed: -(2*3) + (7/5)^2 - log10 became {other_combination}"
    );
}
