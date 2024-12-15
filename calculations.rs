use crate::data_processing::CleanRecord;
use std::error::Error;

use statrs::distribution::{ContinuousCDF, StudentsT}; // Add ContinuousCDF to imports

pub fn perform_t_test(data1: &[f64], data2: &[f64]) -> Result<(f64, f64), &'static str> {
    if data1.is_empty() || data2.is_empty() {
        return Err("One or both datasets are empty");
    }

    let mean1 = data1.iter().sum::<f64>() / data1.len() as f64;
    let mean2 = data2.iter().sum::<f64>() / data2.len() as f64;

    let var1 = data1.iter().map(|x| (x - mean1).powi(2)).sum::<f64>() / (data1.len() - 1) as f64;
    let var2 = data2.iter().map(|x| (x - mean2).powi(2)).sum::<f64>() / (data2.len() - 1) as f64;

    let pooled_variance = ((data1.len() - 1) as f64 * var1 + (data2.len() - 1) as f64 * var2)
        / ((data1.len() + data2.len() - 2) as f64);

    let t_stat = (mean1 - mean2)
        / (pooled_variance * (1.0 / data1.len() as f64 + 1.0 / data2.len() as f64).sqrt());

    let degrees_of_freedom = (data1.len() + data2.len() - 2) as f64;

    let t_dist = StudentsT::new(0.0, 1.0, degrees_of_freedom).unwrap();

    // Use the ContinuousCDF trait for the cdf method
    let p_value = 2.0 * (1.0 - t_dist.cdf(t_stat.abs()));

    Ok((t_stat, p_value))
}
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