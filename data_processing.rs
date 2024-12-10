use csv::Reader;
use serde::Deserialize;
use std::error::Error;
#[derive(Debug, Deserialize)]
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
    field
        .trim()                           // Remove any leading/trailing spaces
        .parse::<f64>()                   // Parse the field as a floating-point number
        .unwrap_or(0.0)                   // Default to 0.0 if parsing fails
        .round() as u32                   // Round the float and convert to u32
}
pub fn clean_record(r: DirtyRecord) -> CleanRecord {
    let mut c = CleanRecord::default();

    // Copy jurisdiction directly
    c.jurisdiction = r.jurisdiction.clone();

    // Parse numeric values and handle commas
    c.year = r.year.trim_matches('"').parse::<u32>().unwrap_or(0);
    c.prisoner_count = r.prisoner_count.replace(",", "").parse::<u32>().unwrap_or(0);
    c.state_population = parse_float_to_u32(&r.state_population);
    c.violent_crime_total = parse_float_to_u32(&r.violent_crime_total);
    // Calculate rates
    c.incarceration_rate = if c.state_population > 0 {
        c.prisoner_count as f32 / c.state_population as f32 * 100_000.0
    } else {
        0.0
    };

    c.crime_rate = if c.state_population > 0 {
        c.violent_crime_total as f32 / c.state_population as f32 * 100_000.0
    } else {
        0.0
    };

    c
}
pub fn process_dataset(file_path: &str) -> Result<Vec<CleanRecord>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut clean_records = Vec::new();

    for result in rdr.deserialize() {
        let dirty: DirtyRecord = result?;
        let clean = clean_record(dirty);
        clean_records.push(clean);
    }

    Ok(clean_records)
}

pub fn filter_by_state(records: &[CleanRecord], state: &str) -> Vec<CleanRecord> {
    records
        .iter()
        .filter(|r| r.jurisdiction.to_lowercase() == state.to_lowercase())
        .cloned()
        .collect()
}