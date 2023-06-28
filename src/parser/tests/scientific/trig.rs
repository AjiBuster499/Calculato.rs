use super::*;

#[test]
fn sine_test() {
    let equation = calculate("sin30");

    assert_eq!(
        equation, 0.5,
        "Sine Assertion Failed: sin30 became {equation}"
    );
}

#[test]
fn cosine_test() {
    let equation = calculate("cos90");

    assert_eq!(
        equation, 0.0,
        "Cosine Assertion Failed: cos90 became {equation}"
    );
}

#[test]
fn tangent_test() {
    let equation = calculate("tan45");

    assert_eq!(
        equation, 1.0,
        "Tangent Assertion Failed: tan45 became {equation}"
    );
}
