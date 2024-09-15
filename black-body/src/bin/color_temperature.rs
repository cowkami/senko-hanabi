use plotters::prelude::*;

use black_body::BlackBody;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_path = "artifacts/output/color_temperature.png";
    let width = 1080;
    let height = 720;

    let (x_min, x_max) = (0.0, 10_000.0 as f32);
    let (y_min, y_max) = (0.0, 1.0 as f32);
    let (x_lim, y_lim) = (x_min..x_max, y_min..y_max);

    let root = BitMapBackend::new(output_path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Color temperature", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(x_lim, y_lim)?;

    chart
        .configure_mesh()
        .x_label_formatter(&|x| format!("{x}"))
        .y_label_formatter(&|y| format!("{y}"))
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc("temperature [K]")
        .y_desc("intensity (0.0 - 1.0)")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    let plotting_area = chart.plotting_area();

    let (x_range, y_range) = plotting_area.get_pixel_range();
    let pixel_width = x_range.end - x_range.start;
    let pixel_heght = y_range.end - y_range.start;

    // draw line for each temperature
    for xi in 0..=pixel_width {
        let x = (xi as f32) / pixel_width as f32 * (x_max - x_min);

        let body = BlackBody::new(x as f64);
        let color = body.color_for_eye();

        for yi in 0..=pixel_heght {
            let y = yi as f32 / pixel_heght as f32;
            let intensity = (y / y_max) as f64;
            let color = RGBAColor(
                (255.0 * intensity * color[0]) as u8,
                (255.0 * intensity * color[1]) as u8,
                (255.0 * intensity * color[2]) as u8,
                1.0,
            );

            let _ = plotting_area.draw_pixel((x, y), &color);
        }
    }

    root.present()?;

    Ok(())
}
