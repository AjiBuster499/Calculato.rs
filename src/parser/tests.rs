use super::*;

#[test]
fn basic_arithmetic_test() {
    assert!(
        calculate("2+2") == 4.0,
        "Addition Assertion Failed: 2+2 became {}",
        calculate("2+2")
    );
    assert!(
        calculate("2-2") == 0.0,
        "Subtraction Assertion Failed: 2-2 became {}",
        calculate("2-2")
    );
    assert!(
        calculate("2*2") == 4.0,
        "Multiplication Assertion Failed: 2*2 became {}",
        calculate("2*2")
    );
    assert!(
        calculate("2/2") == 1.0,
        "Division Assertion Failed: 2/2 became {}",
        calculate("2/2")
    );
}

#[test]
fn complex_arithmetic_test() {
    let equation1 = "2+2*100-100";
    let equation2 = "0*-1+3/4+8^2";
    let answer1 = calculate(equation1);
    let answer2 = calculate(equation2);

    assert_eq!(
        answer1, 102_f32,
        "Order of Operations Assertion failed: \
            2 + 2 * 100 - 100 became: {answer1}"
    );

    assert_eq!(
        answer2, 64.75_f32,
        "Negative and Powers Assertion failed: \
            0 * -1 + 3 / 4 + 8^2 became: {answer2}"
    );
}

#[test]
#[should_panic]
fn styling_test() {
    let solo_negative = calculate("-1");
    assert_eq!(
        solo_negative, -1_f32,
        "Solo Negative Assertion Failed: -1 became {solo_negative}"
    );

    let decimal_with_no_integer_prefix = calculate(".1");
    assert_eq!(
        decimal_with_no_integer_prefix, 0.1_f32,
        "Decimal with no Integer Prefix Assertion failed: \
            .1 became {decimal_with_no_integer_prefix}"
    );
}

#[test]
fn scientific_test() {
    let logarithm = calculate("log10");
    assert_eq!(
        logarithm, 1_f32,
        "Logarithm_10 Assertion failed: log10 became {logarithm}"
    );

    let natural_log = calculate("ln10");
    assert_eq!(
        natural_log,
        10_f32.ln(),
        "Natural Logarithm Assertion failed: ln10 became {natural_log}"
    );
}

#[test]
#[should_panic]
fn factorial_test() {
    let factorial = calculate("10!");
    assert_eq!(
        factorial,
        (10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2) as f32,
        "Factorial Assertion failed: 10! became {factorial}"
    );
}

#[test]
fn combination_test() {
    let combination = calculate("-(2*3)+7/5^2-log10");
    assert_eq!(
        combination,
        -6.72,
        "Combination Assertion failed: -(2*3) + 7/(5^2) - log10 became {combination}"
    );

    let other_combination = calculate("-(2*3)+(7/5)^2-log10");
    assert_eq!(
        other_combination,
        -5.04,
        "Combination Assertion failed: -(2*3) + (7/5)^2 - log10 became {other_combination}"
    );
}
