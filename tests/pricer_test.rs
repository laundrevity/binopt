use pricer::{call};

const EPSILON: f64 = 1e-6;

#[test]
fn test_call() {
    let s = 100.0;
    let k = 110.0;
    let r = 0.05;
    let t = 1.0;
    let sigma = 0.25;
    let q = 0.03;
    let n = 100;

    let call_price = call(t, s, k, r, sigma, q, n);
    let expected_call_price = 14.800026925784081;

    assert!(
        (call_price - expected_call_price).abs() < EPSILON,
        "Expected call price: {}, got: {}",
        expected_call_price,
        call_price
    );
}