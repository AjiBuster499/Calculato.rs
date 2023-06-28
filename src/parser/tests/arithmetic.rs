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
        answer1, 102_f64,
        "Order of Operations Assertion failed: \
            2 + 2 * 100 - 100 became: {answer1}"
    );

    assert_eq!(
        answer2, 64.75_f64,
        "Negative and Powers Assertion failed: \
            0 * -1 + 3 / 4 + 8^2 became: {answer2}"
    );
}
