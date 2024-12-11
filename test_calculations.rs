
use mass_incarceration_analysis::calculations::linear_regression;
use mass_incarceration_analysis::data_processing::CleanRecord;

#[test]
fn test_linear_regression() {
    let records = vec![
        CleanRecord {
            jurisdiction: "ARIZONA".to_string(),
            year: 2001,
            prisoner_count: 27710,
            state_population: 5306966,
            violent_crime_total: 28675,
            incarceration_rate: 522.1439,
            crime_rate: 540.3276,
        },
        CleanRecord {
            jurisdiction: "ARIZONA".to_string(),
            year: 2002,
            prisoner_count: 29359,
            state_population: 5441125,
            violent_crime_total: 30171,
            incarceration_rate: 539.5759,
            crime_rate: 554.4993,
        },
    ];

    let result = linear_regression(&records);
    assert!(result.is_ok());
}