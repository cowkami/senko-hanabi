use black_body::BlackBody;
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_path = "artifacts/output/color_temperature.png";
    let width = 1080;
    let height = 720;

    let (x_min, x_max) = (0.0, 7000.0 as f32);
    let (y_min, y_max) = (0.0, 1.0 as f32);
    let (x_lim, y_lim) = (x_min..x_max, y_min..y_max);

    let temps = [255.0, 3000.0, 4000.0, 5000.0, 6000.0, 7000.0];

    let root = BitMapBackend::new(output_path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Color temperature", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(70)
        .y_label_area_size(0)
        .build_cartesian_2d(x_lim, y_lim)?;

    chart
        .configure_mesh()
        .y_label_formatter(&|y| format!("{y}"))
        .disable_x_mesh()
        .disable_y_mesh()
        .disable_y_axis()
        .x_desc("temperature [K]")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    let plotting_area = chart.plotting_area();
    let range = plotting_area.get_pixel_range();

    let pixel_heght = range.1.end - range.1.start;
    let pixel_width = range.0.end - range.0.start;

    // draw line for each temperature
    for xi in 0..=pixel_width {
        let body = BlackBody::new(xi as f64);
        let color = body.color_for_eye();
        println!("raw color: {:?}", color);
        // let norm: f64 = color[0..3].into_iter().sum();
        // let color = color.iter().map(|&x| x / norm).collect::<Vec<f64>>();

        println!("normalized color: {:?}", color);

        let color = color
            .iter()
            .map(|&x| (x * 255.0) as u8)
            .collect::<Vec<u8>>();
        let color = RGBAColor(color[0], color[1], color[2], 1.0);

        for yi in 0..=pixel_heght {
            let y = yi as f32 / pixel_heght as f32;
            let x = xi as f32 / pixel_width as f32 * x_max;

            let _ = plotting_area.draw_pixel((x, y as f32), &color);
        }
    }
    root.present()?;

    Ok(())
}
