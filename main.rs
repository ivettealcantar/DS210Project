use mass_incarceration_analysis::{
    process_dataset, filter_by_state, construct_graph, compute_degree_centrality,
    compute_average_shortest_path, compute_k_core, group_states_by_centrality,
    linear_regression, plot_rates, plot_degree_centrality, plot_trends_over_time,
    plot_national_averages, nonlinear_regression, export_graph, identify_outliers, compare_states,
    plot_crime_rates_comparison, perform_t_test,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _start = std::time::Instant::now();

    // Check dataset existence
    if !std::path::Path::new("crime_and_incarceration_by_state.csv").exists() {
        eprintln!("Dataset file not found. Please ensure the file is present.");
        return Ok(());
    }

    // Step 1: Process the dataset
    println!("Processing dataset...");
    let (records, invalid_records) = process_dataset("crime_and_incarceration_by_state.csv")?;

    // Log invalid records
    if !invalid_records.is_empty() {
        println!("The following records have missing or invalid data:");
        for record in &invalid_records {
            println!("{:?}", record);
        }
    }

    // If no valid records, terminate the program
    if records.is_empty() {
        eprintln!("No valid records found. Exiting.");
        return Ok(());
    }

    // Step 2: Perform linear regression
    println!("Performing linear regression...");
    linear_regression(&records)?;

    // Step 3: Plot average rates
    println!("Plotting average rates...");
    plot_rates(&records)?;

    // Step 4: Perform nonlinear regression
    println!("Performing nonlinear regression...");
    nonlinear_regression(&records)?;

    // Step 5: Filter data for specific states and plot trends
    for state in &["Arizona", "Massachusetts"] {
        let state_data = filter_by_state(&records, state);
        if state_data.is_empty() {
            eprintln!("No data found for {}.", state);
        } else {
            plot_trends_over_time(&records, Some(state))?;
        }
    }

    // Step 6: Construct and export graph
    println!("Constructing graph...");
    let graph = construct_graph(&records);
    println!("Graph has {} nodes and {} edges.", graph.node_count(), graph.edge_count());

    println!("Exporting graph...");
    export_graph(&graph, "graph.dot")?;
    println!(
        "Graph exported to 'graph.dot'. Use the following command to visualize:\n\
         dot -Tpng graph.dot -o graph.png"
    );

    // Step 7: Compute degree centrality and plot
    println!("Computing degree centrality...");
    let degree_centrality = compute_degree_centrality(&graph);
    println!("\n--- Degree Centrality ---");
    for (state, degree) in &degree_centrality {
        println!("State: {:<15} | Degree: {}", state, degree);
    }
    plot_degree_centrality(&degree_centrality)?;

    // Step 8: Analyze shortest path and k-core
    let avg_shortest_path = compute_average_shortest_path(&graph);
    println!("Average Shortest Path Length: {:.4}", avg_shortest_path);

    let k_core = compute_k_core(&graph, 3);
    println!("3-Core Subgraph Nodes: {:?}", k_core);

    // Step 9: Categorize states by centrality
    let (high, medium, low) = group_states_by_centrality(&degree_centrality);
    println!("High Centrality States: {:?}", high);
    println!("Medium Centrality States: {:?}", medium);
    println!("Low Centrality States: {:?}", low);

    // Step 10: Plot nationwide trends and identify outliers
    println!("Plotting nationwide trends...");
    plot_national_averages(&records)?;

    let outliers = identify_outliers(&records);
    println!("Outliers: {:?}", outliers);

    // Step 11: Compare Arizona and Massachusetts crime rates
    let (arizona_data, massachusetts_data) = compare_states(&records, "Arizona", "Massachusetts");
    println!("Arizona Data: {:?}", arizona_data);
    println!("Massachusetts Data: {:?}", massachusetts_data);

    plot_crime_rates_comparison(&records, &["Arizona", "Massachusetts"])?;

    // Step 12: Perform t-test
    let crime_rates_az: Vec<f64> = arizona_data.iter().map(|r| r.crime_rate as f64).collect();
    let crime_rates_ma: Vec<f64> = massachusetts_data.iter().map(|r| r.crime_rate as f64).collect();

    let (t_stat, p_value) = perform_t_test(&crime_rates_az, &crime_rates_ma)?;
    println!(
        "T-Test Results: t-statistic = {:.4}, p-value = {:.4}",
        t_stat, p_value
    );

    Ok(())
}