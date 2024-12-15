pub mod data_processing;
pub mod calculations;
pub mod visualization;
pub mod nonlinear;
pub mod petgraph_vis;
pub mod graph_analysis;
pub mod state_comparison;

// Re-export commonly used items for easier access in main.rs
pub use data_processing::{process_dataset, filter_by_state, identify_outliers,CleanRecord};
pub use graph_analysis::{construct_graph, compute_degree_centrality, compute_average_shortest_path, compute_k_core, group_states_by_centrality};
pub use calculations::linear_regression;
pub use visualization::{plot_rates, plot_degree_centrality, plot_trends_over_time, plot_national_averages, plot_crime_rates_comparison};
pub use nonlinear::nonlinear_regression;
pub use petgraph_vis::{construct_similarity_graph, cluster_states, plot_graph_with_clusters, export_graph, visualize_similarity_graph, export_graph_to_png};
pub use state_comparison::compare_states;
// Optionally, add a shared run function for testing or execution
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (valid_records, invalid_records) = data_processing::process_dataset("crime_and_incarceration_by_state.csv")?;

    // Log invalid records
    if !invalid_records.is_empty() {
        println!("The following records have missing or invalid data:");
        for record in &invalid_records {
            println!("{:?}", record);
        }
    }

    if valid_records.is_empty() {
        eprintln!("No valid records found. Exiting.");
        return Ok(());
    }

    // Pass only valid records to linear_regression
    calculations::linear_regression(&valid_records)?;

    Ok(())
}