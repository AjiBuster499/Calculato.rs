use super::*;

mod arithmetic;
mod scientific;

#[test]
#[should_panic]
fn styling_test() {
    let solo_negative = calculate("-1");
    assert_eq!(
        solo_negative, -1_f64,
        "Solo Negative Assertion Failed: -1 became {solo_negative}"
    );

    let decimal_with_no_integer_prefix = calculate(".1");
    assert_eq!(
        decimal_with_no_integer_prefix, 0.1_f64,
        "Decimal with no Integer Prefix Assertion failed: \
            .1 became {decimal_with_no_integer_prefix}"
    );
}
