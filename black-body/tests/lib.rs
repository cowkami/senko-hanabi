use black_body::BlackBody;
use rstest::rstest;

#[rstest]
#[case(1.0)]
#[should_panic]
#[case(-1.0)]
fn calc_wave_length(#[case] temperature: f64) {
    let _ = BlackBody::new(temperature);
}
