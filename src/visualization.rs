use plotters::prelude::*;

pub fn plot_signal(signal: Vec<i32>, file_name: &str) {
    let root = BitMapBackend::new(file_name, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Line Coding Signal", ("sans-serif", 50))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..signal.len(), -2..2)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            signal.iter().enumerate().map(|(i, &v)| (i, v)),
            &RED,
        ))
        .unwrap();
}
