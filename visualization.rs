use crate::data_processing::CleanRecord;
use plotters::prelude::*;
use std::error::Error;
//use plotly::{Bar, Plot};



pub fn plot_degree_centrality(degree_centrality: &[(String, usize)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("degree_centrality_chart.png", (1600, 1200)).into_drawing_area();
    root.fill(&WHITE)?;

    let states: Vec<_> = degree_centrality.iter().map(|(state, _)| state.clone()).collect();
    let degrees: Vec<_> = degree_centrality.iter().map(|(_, degree)| *degree).collect();

    let max_degree = *degrees.iter().max().unwrap_or(&0);

    let mut chart = ChartBuilder::on(&root)
        .caption("Degree Centrality by State", ("Arial", 30))
        .x_label_area_size(300) // Allocate more vertical space for labels
        .y_label_area_size(60)
        .build_cartesian_2d(0..states.len(), 0..max_degree + 10)?;

    chart.configure_mesh()
        .x_labels(states.len()) // Show all state labels
        .x_label_formatter(&|idx| states.get(*idx).unwrap_or(&"UNK".to_string()).to_string())
        .x_label_style(
            TextStyle::from(("Arial", 12).into_font())
                .color(&BLACK)
                .transform(FontTransform::Rotate90), // Rotate labels 90 degrees
        )
        .y_desc("Degree Centrality")
        .x_desc("States")
        .draw()?;

    chart.draw_series(states.iter().enumerate().map(|(idx, _)| {
        let degree = degrees[idx];
        Rectangle::new([(idx, 0), (idx + 1, degree)], BLUE.filled())
    }))?;

    println!("Degree centrality chart saved to 'degree_centrality_chart.png'.");
    Ok(())
}

pub fn plot_rates(records: &[CleanRecord]) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new("rates.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let avg_incarceration: f32 = records.iter().map(|r| r.incarceration_rate).sum::<f32>() / records.len() as f32;
    let avg_crime: f32 = records.iter().map(|r| r.crime_rate).sum::<f32>() / records.len() as f32;

    let mut chart = ChartBuilder::on(&root)
        .caption("Average Rates", ("Arial", 20))
        .x_label_area_size(50)
        .y_label_area_size(40)
        .build_cartesian_2d(0..2, 0f32..avg_incarceration.max(avg_crime) * 1.2)?;

    chart.configure_mesh()
        .x_labels(2)
        .x_label_formatter(&|x| match *x {
            0 => "Incarceration".to_string(),
            1 => "Crime".to_string(),
            _ => "".to_string(),
        })
        .y_desc("Rates")
        .draw()?;

    chart.draw_series(vec![
        Rectangle::new([(0, 0.0), (1, avg_incarceration)], BLUE.mix(0.8).filled()),
        Rectangle::new([(1, 0.0), (2, avg_crime)], BLUE.mix(0.8).filled()),
    ])?;

    println!("Bar chart saved to 'output/rates.png'");

    Ok(())
}