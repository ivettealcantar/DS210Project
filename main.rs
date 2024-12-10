mod data_processing;
mod calculations;
mod visualization;
mod nonlinear;

mod petgraph_vis;
mod graph_analysis;

use data_processing::{process_dataset, filter_by_state};
use graph_analysis::{construct_graph, compute_degree_centrality, compute_average_shortest_path, compute_k_core, group_states_by_centrality};
use calculations::linear_regression;
use visualization::{plot_rates, plot_degree_centrality};
use nonlinear::nonlinear_regression;

use petgraph_vis::export_graph;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();

    // Step 1: Process the dataset
    println!("Processing dataset...");
    let records = process_dataset("crime_and_incarceration_by_state.csv")?;
    if records.is_empty() {
        eprintln!("The dataset is empty. Please check your CSV file.");
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

    // Step 5: Filter data for specific states
    println!("Filtering data for Arizona and Massachusetts...");
    let arizona_data = filter_by_state(&records, "Arizona");
    if arizona_data.is_empty() {
        eprintln!("No data found for Arizona.");
    } else {
        println!("Arizona Data: {:?}", arizona_data);
    }

    let massachusetts_data = filter_by_state(&records, "Massachusetts");
    if massachusetts_data.is_empty() {
        eprintln!("No data found for Massachusetts.");
    } else {
        println!("Massachusetts Data: {:?}", massachusetts_data);
    }

    // Step 6: Construct graph
    println!("Constructing graph...");
    let graph = construct_graph(&records);
    println!("Graph has {} nodes and {} edges.", graph.node_count(), graph.edge_count());

    // Step 7: Export graph for visualization
    println!("Exporting graph...");
    export_graph(&graph, "graph.dot")?;
    println!("Graph exported to 'graph.dot'. Use Graphviz to visualize.");

    // Step 8: Compute degree centrality and plot
    println!("Computing degree centrality...");
    let degree_centrality = compute_degree_centrality(&graph);
    println!("\n--- Degree Centrality ---");
    for (state, degree) in &degree_centrality {
        println!("State: {:<15} | Degree: {}", state, degree);
    }
    plot_degree_centrality(&degree_centrality)?;

    // Step 9: Analyze shortest path and k-core
    let avg_shortest_path = compute_average_shortest_path(&graph);
    println!("Average Shortest Path Length: {:.4}", avg_shortest_path);

    let k_core = compute_k_core(&graph, 3);
    println!("3-Core Subgraph Nodes: {:?}", k_core);

    println!("Program completed in {:.2?}", start.elapsed());
    

    let (high, medium, low) = group_states_by_centrality(&degree_centrality);
    println!("High Centrality States: {:?}", high);
    println!("Medium Centrality States: {:?}", medium);
    println!("Low Centrality States: {:?}", low);
    Ok(())

}
