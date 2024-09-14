use black_body::BlackBody;
use rstest::rstest;

const EPS: f64 = 1.0e-10;

#[rstest]
#[case(1.0)]
#[should_panic]
#[case(-1.0)]
fn new(#[case] temperature: f64) {
    let _ = BlackBody::new(temperature);
}

// ref: https://www.chegg.com/homework-help/questions-and-answers/incandescent-lightbulb-desired-emit-least-15-percent-energy-wavelengths-shorter-08-m-deter-q17136835
#[rstest]
#[case(200.0, 1.0, 0.0)]
#[case(600.0, 1.0, 0.0)]
#[case(1000.0, 1.0, 3.21e-4)]
fn radiance(#[case] temperature: f64, #[case] wave_length: f64, #[case] expected: f64) {
    let body = BlackBody::new(temperature);
    let result = body.radiance(wave_length);
    assert!(
        (expected - result).abs() / result < EPS,
        "expected: {}, got: {}",
        expected,
        result
    );
}
