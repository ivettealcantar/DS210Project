use plotters::prelude::*;
use crate::data_processing::CleanRecord;

pub fn national_trends(records: &[CleanRecord]) -> Result<(), Box<dyn std::error::Error>> {
    let mut yearly_data: std::collections::HashMap<u32, (u32, u32)> = std::collections::HashMap::new();

    for record in records {
        let entry = yearly_data.entry(record.year).or_insert((0, 0));
        entry.0 += record.prisoner_count;
        entry.1 += record.violent_crime_total;
    }

    let mut years: Vec<u32> = Vec::new();
    let mut total_prisoners: Vec<u32> = Vec::new();
    let mut total_crimes: Vec<u32> = Vec::new();

    for (year, (prisoners, crimes)) in &yearly_data {
        years.push(*year);
        total_prisoners.push(*prisoners);
        total_crimes.push(*crimes);
    }

    let root = BitMapBackend::new("output/national_trends.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = *total_prisoners.iter().max().unwrap_or(&1).max(total_crimes.iter().max().unwrap_or(&1)) as f64;

    let mut chart = ChartBuilder::on(&root)
        .caption("National Trends: Incarceration vs Crime", ("Arial", 20))
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(years[0] as f64..*years.last().unwrap() as f64, 0.0..max_y)?;

    chart.configure_mesh().x_labels(10).y_labels(10).draw()?;

    chart
        .draw_series(LineSeries::new(
            years.iter().map(|&year| year as f64).zip(total_prisoners.iter().map(|&p| p as f64)),
            &RED,
        ))?
        .label("Prisoners")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            years.iter().map(|&year| year as f64).zip(total_crimes.iter().map(|&c| c as f64)),
            &BLUE,
        ))?
        .label("Crimes")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart.configure_series_labels().border_style(&BLACK).draw()?;

    println!("National trends plot saved to 'output/national_trends.png'");
    Ok(())
}