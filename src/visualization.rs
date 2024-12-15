use crate::data_processing::CleanRecord;
use plotters::prelude::*;
use std::error::Error;
use std::collections::HashMap;
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
pub fn plot_national_averages(records: &[CleanRecord]) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Calculate national averages
    let mut yearly_data: HashMap<u32, (f32, f32, usize)> = HashMap::new(); // year -> (total_incarceration_rate, total_crime_rate, count)

    for record in records {
        let entry = yearly_data.entry(record.year).or_insert((0.0, 0.0, 0));
        entry.0 += record.incarceration_rate;
        entry.1 += record.crime_rate;
        entry.2 += 1;
    }

    let mut years: Vec<u32> = yearly_data.keys().cloned().collect();
    years.sort(); // Ensure years are in order

    let averages: Vec<(f32, f32)> = years
        .iter()
        .map(|year| {
            let (total_incarceration_rate, total_crime_rate, count) = yearly_data[year];
            (
                total_incarceration_rate / count as f32, // Average incarceration rate
                total_crime_rate / count as f32,         // Average crime rate
            )
        })
        .collect();

    // Step 2: Prepare the plot
    let root = BitMapBackend::new("national_averages.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_rate = averages.iter().map(|(inc, crime)| inc.max(*crime)).fold(0.0, f32::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("National Averages (2001-2016)", ("Arial", 20))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(2001u32..2016u32, 0f32..(max_rate * 1.1))?;

    chart.configure_mesh()
        .x_desc("Year")
        .y_desc("Rate")
        .draw()?;

    // Step 3: Plot incarceration and crime rates
    chart.draw_series(LineSeries::new(
        years.iter().zip(averages.iter()).map(|(year, (inc_rate, _))| (*year, *inc_rate)),
        &BLUE,
    ))?
    .label("Incarceration Rate")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));

    chart.draw_series(LineSeries::new(
        years.iter().zip(averages.iter()).map(|(year, (_, crime_rate))| (*year, *crime_rate)),
        &RED,
    ))?
    .label("Crime Rate")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));

    // Step 4: Add legend
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    println!("Plot saved as 'national_averages.png'.");
    Ok(())
}
pub fn plot_trends_over_time(records: &[CleanRecord], state: Option<&str>) -> Result<(), Box<dyn Error>> {
    // Filter records for the specified state, if provided
    let filtered_records: Vec<&CleanRecord> = match state {
        Some(state_name) => records
            .iter()
            .filter(|r| r.jurisdiction.to_lowercase() == state_name.to_lowercase())
            .collect(),
        None => records.iter().collect(),
    };

    if filtered_records.is_empty() {
        println!("No data available for the specified state.");
        return Ok(());
    }

    // Extract year, incarceration_rate, and crime_rate
    let years: Vec<u32> = filtered_records.iter().map(|r| r.year).collect();
    let incarceration_rates: Vec<f32> = filtered_records.iter().map(|r| r.incarceration_rate).collect();
    let crime_rates: Vec<f32> = filtered_records.iter().map(|r| r.crime_rate).collect();

    // Set up the chart
    let file_name = if let Some(state_name) = state {
        format!("{}_trends_over_time.png", state_name.to_lowercase())
    } else {
        "nationwide_trends_over_time.png".to_string()
    };

    let root = BitMapBackend::new(&file_name, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = incarceration_rates
        .iter()
        .chain(crime_rates.iter())
        .copied()
        .fold(0.0 / 0.0, f32::max); // Find max for y-axis

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!(
                "{} Trends Over Time",
                state.unwrap_or("Nationwide")
            ),
            ("Arial", 30),
        )
        .x_label_area_size(40)
        .y_label_area_size(50)
        .margin(10)
        .build_cartesian_2d(
            *years.first().unwrap()..*years.last().unwrap(),
            0.0..(max_y * 1.2),
        )?;

    chart.configure_mesh().x_desc("Year").y_desc("Rate").draw()?;

    // Draw incarceration rate line
    chart
        .draw_series(LineSeries::new(
            years.iter().zip(incarceration_rates.iter()).map(|(&x, &y)| (x, y)),
            &BLUE,
        ))?
        .label("Incarceration Rate")
        .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &BLUE));

    // Draw crime rate line
    chart
        .draw_series(LineSeries::new(
            years.iter().zip(crime_rates.iter()).map(|(&x, &y)| (x, y)),
            &RED,
        ))?
        .label("Crime Rate")
        .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &RED));

    // Add a legend
    chart.configure_series_labels().background_style(&WHITE).draw()?;

    println!("Trend chart saved to '{}'", file_name);
    Ok(())
    }
    pub fn plot_crime_rates_comparison(records: &[CleanRecord], states: &[&str]) -> Result<(), Box<dyn Error>> {
        // Filter records for the specified states
        let filtered_records: HashMap<String, Vec<&CleanRecord>> = states
            .iter()
            .map(|&state_name| {
                (
                    state_name.to_string(),
                    records
                        .iter()
                        .filter(|r| r.jurisdiction.to_lowercase() == state_name.to_lowercase())
                        .collect(),
                )
            })
            .collect();
    
        if filtered_records.values().all(|v| v.is_empty()) {
            println!("No data available for the specified states.");
            return Ok(());
        }
    
        // Set up the chart
        let file_name = "az_mass_crime_rates_comparison.png";
        let root = BitMapBackend::new(&file_name, (1200, 800)).into_drawing_area();
        root.fill(&WHITE)?;
    
        // Find the maximum year and crime rate for the y-axis
        let max_year = records.iter().map(|r| r.year).max().unwrap_or(0);
        let min_year = records.iter().map(|r| r.year).min().unwrap_or(0);
        let max_crime_rate = records
            .iter()
            .map(|r| r.crime_rate)
            .fold(f32::MIN, f32::max);
    
        let mut chart = ChartBuilder::on(&root)
            .caption("Crime Rates: Arizona vs. Massachusetts", ("Arial", 30))
            .x_label_area_size(40)
            .y_label_area_size(50)
            .margin(10)
            .build_cartesian_2d(min_year..max_year, 0.0..(max_crime_rate * 1.2))?;
    
        chart.configure_mesh().x_desc("Year").y_desc("Crime Rate").draw()?;
    
        // Plot crime rates for each state
        let colors = &[&BLUE, &RED];
        for (i, (state, data)) in filtered_records.iter().enumerate() {
            let years: Vec<u32> = data.iter().map(|r| r.year).collect();
            let crime_rates: Vec<f32> = data.iter().map(|r| r.crime_rate).collect();
    
            chart
                .draw_series(LineSeries::new(
                    years.into_iter().zip(crime_rates.into_iter()),
                    *colors.get(i % colors.len()).unwrap(),
                ))?
                .label(format!("{} Crime Rate", state))
                .legend(move |(x, y)| PathElement::new([(x, y), (x + 20, y)], *colors.get(i % colors.len()).unwrap()));
        }
    
        // Add a legend
        chart.configure_series_labels().background_style(&WHITE).draw()?;
    
        println!("Crime rate comparison chart saved to '{}'", file_name);
    
        Ok(())
    }