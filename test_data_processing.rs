
#[cfg(test)]
mod tests {
    // Import the function to test
    use mass_incarceration_analysis::linear_regression;
    use mass_incarceration_analysis::CleanRecord;

    #[test]
    fn test_linear_regression() {
        // Create a small dataset of CleanRecord
        let records = vec![
            CleanRecord {
                jurisdiction: "State1".to_string(),
                year: 2000,
                prisoner_count: 500,
                state_population: 1_000_000,
                violent_crime_total: 2000,
                incarceration_rate: 50.0,
                crime_rate: 200.0,
            },
            CleanRecord {
                jurisdiction: "State2".to_string(),
                year: 2001,
                prisoner_count: 1000,
                state_population: 1_000_000,
                violent_crime_total: 3000,
                incarceration_rate: 100.0,
                crime_rate: 300.0,
            },
            CleanRecord {
                jurisdiction: "State3".to_string(),
                year: 2002,
                prisoner_count: 1500,
                state_population: 1_000_000,
                violent_crime_total: 4000,
                incarceration_rate: 150.0,
                crime_rate: 400.0,
            },
        ];

        // Call the function
        let result = linear_regression(&records);

        // Assert the result is OK
        assert!(result.is_ok());

        // (Optional) Refactor linear_regression to return values for direct validation
    }
}
