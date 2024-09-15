use std::iter::zip;

use plotters::prelude::*;

use black_body::spectrum::ColorFunction;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_path = "artifacts/output/cie1931_spectrum.png";
    let width = 1080;
    let height = 720;

    let xlim = 300f32..800f32;
    let ylim = 0f32..2f32;

    let wavelengths = (0..=1000).into_iter().map(|x| x as f64); // [nm]
    let color_funcs = [ColorFunction::x, ColorFunction::y, ColorFunction::z];
    let colors = [&RED, &GREEN, &BLUE];

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

    for (func, color) in zip(color_funcs, colors) {
        let data = wavelengths
            .clone()
            .into_iter()
            .map(|x| (x as f32, func(x) as f32));

        chart
            .draw_series(LineSeries::new(
                data,
                ShapeStyle {
                    color: (*color).to_rgba(),
                    filled: false,
                    stroke_width: 1,
                },
            ))?
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
