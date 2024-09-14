use std::iter::zip;

use black_body::BlackBody;
use full_palette::{ORANGE, ORANGE_900, PURPLE};
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_path = "artifacts/output/blackbody_spectrum.png";
    let width = 1080;
    let height = 720;

    let xlim = (100f32..10_000f32).log_scale();
    let ylim = (10f32..100_000f32).log_scale();

    let wavelengths = (0..=10_000).into_iter().map(|x| x as f64); // [nm]
    let temps = [255.0, 3000.0, 4000.0, 5000.0, 6000.0, 7000.0];
    let colors = [&BLACK, &RED, &ORANGE, &ORANGE_900, &BLUE, &PURPLE];

    let root = BitMapBackend::new(output_path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Black body radiance spectrum", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(70)
        .y_label_area_size(70)
        .build_cartesian_2d(xlim, ylim)?;

    chart
        .configure_mesh()
        .y_label_formatter(&|y| format!("{y}"))
        .x_labels(15)
        .y_labels(5)
        .x_desc("wavelength [nm]")
        .y_desc("radiance [kW m^-2 sr^-1 μm^-1]")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    for (temperature, color) in zip(temps, colors) {
        let body = BlackBody::new(temperature);
        let data = wavelengths
            .clone()
            .into_iter()
            .map(|x| (x as f32, (body.radiance(x * 1.0e-9) * 1.0e-9) as f32));

        chart
            .draw_series(LineSeries::new(
                data,
                ShapeStyle {
                    color: (*color).to_rgba(),
                    filled: false,
                    stroke_width: 1,
                },
            ))?
            .label(format!("{temperature} K"))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], *color));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
