use csv::Reader;
use serde::Deserialize;
use std::error::Error;
#[derive(Debug, Deserialize,Clone)]
#[serde(rename_all = "lowercase")]

pub struct DirtyRecord {
    pub jurisdiction: String,
    pub year: String,
    pub prisoner_count: String,
    pub state_population: String,
    pub violent_crime_total: String,
}

#[derive(Debug, Default,Clone)]
pub struct CleanRecord {
    pub jurisdiction: String,
    pub year: u32,
    pub prisoner_count: u32,
    pub state_population: u32,
    pub violent_crime_total: u32,
    pub incarceration_rate: f32,
    pub crime_rate: f32,
}
fn parse_float_to_u32(field: &str) -> u32 {
    if field.trim().is_empty() {
        return 0; // Return a default value for empty strings
    }

    field
        .trim()                           // Remove any leading/trailing spaces
        .parse::<f64>()                   // Parse the field as a floating-point number
        .unwrap_or(0.0)                   // Default to 0.0 if parsing fails
        .round() as u32                   // Round the float and convert to u32
}

pub fn clean_record(r: DirtyRecord) -> Option<CleanRecord> {
    if r.state_population.trim().is_empty() || r.violent_crime_total.trim().is_empty() {
        eprintln!(
            "Missing or invalid data: State = {}, Year = {}",
            r.jurisdiction, r.year
        );
        return None; // Skip records with missing data
    }

    let mut c = CleanRecord::default();
    c.jurisdiction = r.jurisdiction.clone();
    c.year = r.year.trim_matches('"').parse::<u32>().unwrap_or_else(|_| {
        eprintln!("Invalid year format: {}", r.year);
        0
    });
    c.prisoner_count = r.prisoner_count.replace(",", "").parse::<u32>().unwrap_or_else(|_| {
        eprintln!("Invalid prisoner_count: {}", r.prisoner_count);
        0
    });
    c.state_population = parse_float_to_u32(&r.state_population);
    c.violent_crime_total = parse_float_to_u32(&r.violent_crime_total);

    if c.state_population == 0 || c.violent_crime_total == 0 {
        eprintln!(
            "Parsed invalid data: State = {}, Year = {}, State Population = {}, Violent Crime Total = {}",
            r.jurisdiction, c.year, c.state_population, c.violent_crime_total
        );
        return None;
    }

    // Calculate rates
    c.incarceration_rate = c.prisoner_count as f32 / c.state_population as f32 * 100_000.0;
    c.crime_rate = c.violent_crime_total as f32 / c.state_population as f32 * 100_000.0;

    Some(c)
}
pub fn process_dataset(file_path: &str) -> Result<(Vec<CleanRecord>, Vec<DirtyRecord>), Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut clean_records = Vec::new();
    let mut invalid_records = Vec::new();

    for result in rdr.deserialize() {
        let dirty: DirtyRecord = result?;
        match clean_record(dirty.clone()) {
            Some(clean) => clean_records.push(clean),
            None => invalid_records.push(dirty),
        }
    }

    Ok((clean_records, invalid_records))
}

pub fn filter_by_state(records: &[CleanRecord], state: &str) -> Vec<CleanRecord> {
    records
        .iter()
        .filter(|r| r.jurisdiction.to_lowercase() == state.to_lowercase())
        .cloned()
        .collect()
}
pub fn identify_outliers(records: &[CleanRecord]) -> Vec<CleanRecord> {
    let mean_incarceration = records.iter().map(|r| r.incarceration_rate).sum::<f32>() / records.len() as f32;
    let std_incarceration = (records.iter().map(|r| (r.incarceration_rate - mean_incarceration).powi(2)).sum::<f32>() / records.len() as f32).sqrt();

    records.iter()
        .filter(|r| (r.incarceration_rate - mean_incarceration).abs() > 3.0 * std_incarceration) // z-score > 3
        .cloned()
        .collect()
}