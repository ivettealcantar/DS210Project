use crate::data_processing::CleanRecord;
use std::error::Error;

pub fn linear_regression(records: &[CleanRecord]) -> Result<(), Box<dyn Error>> {
    // Perform linear regression calculations
    let x: Vec<f32> = records.iter().map(|r| r.incarceration_rate).collect();
    let y: Vec<f32> = records.iter().map(|r| r.crime_rate).collect();

    let mean_x = x.iter().sum::<f32>() / x.len() as f32;
    let mean_y = y.iter().sum::<f32>() / y.len() as f32;

    let slope = x.iter().zip(y.iter()).map(|(xi, yi)| (xi - mean_x) * (yi - mean_y)).sum::<f32>()
        / x.iter().map(|xi| (xi - mean_x).powi(2)).sum::<f32>();

    let intercept = mean_y - slope * mean_x;

    println!("Linear Regression: y = {:.4}x + {:.4}", slope, intercept);

    Ok(())
}