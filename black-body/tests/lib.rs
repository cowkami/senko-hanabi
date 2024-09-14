use black_body::BlackBody;
use rstest::rstest;

#[rstest]
#[case(1.0)]
#[should_panic]
#[case(-1.0)]
fn new(#[case] temperature: f64) {
    let _ = BlackBody::new(temperature);
}

#[rstest]
#[case(5800.0, 5.0)]
fn calc_wave_length(#[case] temperature: f64, #[case] wave_length: f64) {
    let body = BlackBody::new(temperature);
    assert_eq!(body.calc_wave_length(), wave_length);
}
