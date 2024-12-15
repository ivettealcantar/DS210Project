
use crate::data_processing::CleanRecord;
use nalgebra::{DMatrix, DVector};
use plotters::prelude::*;
use std::error::Error;

pub fn nonlinear_regression(records: &[CleanRecord]) -> Result<(), Box<dyn Error>> {
    // Extract x (incarceration_rate) and y (crime_rate)
    let x: Vec<f64> = records.iter().map(|r| r.incarceration_rate as f64).collect();
    let y: Vec<f64> = records.iter().map(|r| r.crime_rate as f64).collect();

    let n = x.len();
    let mut x_matrix = Vec::new();
    for &xi in &x {
        x_matrix.extend_from_slice(&[xi.powi(2), xi, 1.0]);
    }

    // Convert data to nalgebra matrices
    let x_matrix = DMatrix::from_row_slice(n, 3, &x_matrix);
    let y_vector = DVector::from_column_slice(&y);

    // Solve for coefficients: [a, b, c]
    let xtx = &x_matrix.transpose() * &x_matrix;
    let xty = &x_matrix.transpose() * y_vector;
    let coefficients = xtx.try_inverse().unwrap() * xty;

    let a = coefficients[0];
    let b = coefficients[1];
    let c = coefficients[2];

    println!("Nonlinear Model: y = {:.4}x^2 + {:.4}x + {:.4}", a, b, c);

    // Plotting
    let root = BitMapBackend::new("nonlinear_regression.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_x = *x.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_y = *y.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Nonlinear Regression: Incarceration Rate vs Crime Rate", ("Arial", 20))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..max_x, 0.0..max_y)?;

    chart.configure_mesh().draw()?;

    // Plot scatter points
    chart.draw_series(
        x.iter()
            .zip(y.iter())
            .map(|(&xi, &yi)| Circle::new((xi, yi), 3, BLUE.filled())),
    )?;

    // Plot quadratic curve
    chart.draw_series(LineSeries::new(
        (0..1000).map(|i| {
            let xi = i as f64 * max_x / 1000.0;
            let yi = a * xi.powi(2) + b * xi + c;
            (xi, yi)
        }),
        &RED,
    ))?;

    println!("Nonlinear regression plot saved to 'output/nonlinear_regression.png'");

    Ok(())
}