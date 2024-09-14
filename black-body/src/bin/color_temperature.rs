use black_body::BlackBody;
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_path = "artifacts/output/color_temperature.png";
    let width = 1080;
    let height = 720;

    let (x_min, x_max) = (0.0, 16000.0 as f32);
    let (y_min, y_max) = (0.0, 1.0 as f32);
    let (x_lim, y_lim) = (x_min..x_max, y_min..y_max);

    let root = BitMapBackend::new(output_path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Color temperature", ("sans-serif", 40))
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
        println!("{:?}", xi);
        let x = (xi as f32) / pixel_width as f32 * (x_max - x_min);
        println!("{:?}", x);

        // println!("temperature: {:?}", x);
        let body = BlackBody::new(x as f64);
        let color = body.color_for_eye();

        for yi in 0..=pixel_heght {
            let y = yi as f32 / pixel_heght as f32;

            // println!("raw color: {:?}", color);
            let norm: f64 = color[0..3].into_iter().sum();
            let color = if norm < 1.0e-6 {
                BLACK.to_rgba()
            } else {
                let color = color
                    .iter()
                    .map(|&x| 3.0 * (y as f64 / y_max as f64) * x / norm)
                    .collect::<Vec<f64>>();

                // println!("normalized color: {:?}", color);

                let color = color
                    .iter()
                    .map(|&x| (x * 255.0) as u8)
                    .collect::<Vec<u8>>();
                let color = RGBAColor(color[0], color[1], color[2], 1.0);
                // println!("color rgb: {:?}", color);
                color
            };

            let _ = plotting_area.draw_pixel((x, y as f32), &color);
        }
    }
    root.present()?;

    Ok(())
}
