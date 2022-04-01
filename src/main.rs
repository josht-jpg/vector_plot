use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "vector.gif";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let area = BitMapBackend::gif(OUT_FILE_NAME, (600, 400), 100)?.into_drawing_area();

    for pitch in 0..157 {
        area.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&area).build_cartesian_3d(0.0..5.0, 0.0..5.0, 0.0..5.0)?;

        chart.with_projection(|mut p| {
            p.pitch = 1.57 - (1.57 - pitch as f64 / 50.0).abs();
            p.scale = 0.7;
            p.into_matrix()
        });
        chart.configure_axes().draw()?;

        chart.draw_series(PointSeries::of_element(
            vec![(2.5, 3., 4.)],
            5.5,
            &RED,
            &|c, s, st| {
                EmptyElement::at(c)
                    + Circle::new((0, 0), s, st.filled())
                    + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font())
            },
        ))?;

        area.present()?;
    }

    area.present().expect("Unable to write result to file.");
    Ok(())
}

#[test]
fn entry_point() {
    main().unwrap()
}
