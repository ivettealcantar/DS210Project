//use plotters::prelude::*;
use crate::data_processing::CleanRecord;
use std::error::Error;

pub fn diminishing_returns_visualization(records: &[CleanRecord]) -> Result<(), Box<dyn Error>> {
    use plotters::prelude::*;

    // Extract x (incarceration rate) and y (crime rate) values
    let x: Vec<f64> = records.iter().map(|r| r.incarceration_rate as f64).collect();
    let y: Vec<f64> = records.iter().map(|r| r.crime_rate as f64).collect();

    // Determine the range of x and y
    let max_x = x.iter().cloned().fold(0.0 / 0.0, f64::max); // Find max value of x
    let max_y = y.iter().cloned().fold(0.0 / 0.0, f64::max); // Find max value of y

    // Fit a logarithmic curve
    let log_x: Vec<f64> = x.iter().map(|&xi| (xi + 1.0).ln()).collect();
    let coefficients = linregress(&log_x, &y)?; // Fit y = a + b * ln(x)

    let a = coefficients.0; // Intercept
    let b = coefficients.1; // Slope

    println!("Logarithmic Model: y = {:.4} + {:.4}ln(x)", a, b);

    // Save the plot
    let root = BitMapBackend::new("diminishing_returns.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Diminishing Marginal Returns: Crime Rate vs Incarceration Rate", ("Arial", 20))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..max_x, 0.0..max_y)?;

    chart.configure_mesh().draw()?;

    // Plot the scatter points
    chart.draw_series(
        x.iter()
            .zip(y.iter())
            .map(|(&xi, &yi)| Circle::new((xi, yi), 3, BLUE.filled())),
    )?;

    // Plot the fitted logarithmic curve
    chart.draw_series(LineSeries::new(
        (1..1000).map(|i| {
            let xi = i as f64 * max_x / 1000.0;
            let yi = a + b * (xi + 1.0).ln();
            (xi, yi)
        }),
        &RED,
    ))?;

    println!("Visualization saved to 'output/diminishing_returns.png'");

    Ok(())
}

// Helper function to perform linear regression
fn linregress(x: &[f64], y: &[f64]) -> Result<(f64, f64), Box<dyn Error>> {
    let n = x.len() as f64;
    let sum_x = x.iter().sum::<f64>();
    let sum_y = y.iter().sum::<f64>();
    let sum_x_squared = x.iter().map(|&xi| xi * xi).sum::<f64>();
    let sum_xy = x.iter().zip(y.iter()).map(|(&xi, &yi)| xi * yi).sum::<f64>();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x_squared - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / n;

    Ok((intercept, slope))
}