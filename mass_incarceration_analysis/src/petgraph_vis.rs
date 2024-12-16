use crate::data_processing::CleanRecord;
use petgraph::dot::{Dot, Config};
use petgraph::graph::UnGraph;
use std::fs::{self, File};
use std::io::Write;



pub fn visualize_similarity_graph(graph: &UnGraph<String, f32>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Generate the graph in DOT format
    let dot_format = Dot::with_config(&graph, &[Config::EdgeNoLabel]);

    // Save to file
    fs::write(filename, format!("{:?}", dot_format))?;
    println!("Graph visualization saved to '{}'", filename);

    Ok(())
}
pub fn construct_similarity_graph(records: &[CleanRecord]) -> UnGraph<String, f32> {
    let mut graph = UnGraph::new_undirected();

    // Map node names to indices
    let nodes: Vec<_> = records
        .iter()
        .map(|record| graph.add_node(record.jurisdiction.clone())) // Store NodeIndex
        .collect();

    for i in 0..records.len() {
        for j in (i + 1)..records.len() {
            let sim = calculate_similarity(&records[i], &records[j]);
            if sim > 0.7 {
                graph.add_edge(nodes[i], nodes[j], sim); // Use NodeIndex
            }
        }
    }

    graph
}

pub fn calculate_similarity(record1: &CleanRecord, record2: &CleanRecord) -> f32 {
    let crime_diff = (record1.crime_rate - record2.crime_rate).abs();
    let incarceration_diff = (record1.incarceration_rate - record2.incarceration_rate).abs();
    let max_diff = (record1.crime_rate + record2.crime_rate + record1.incarceration_rate + record2.incarceration_rate).max(1.0);
    1.0 - (crime_diff + incarceration_diff) / max_diff
}



pub fn export_graph(graph: &petgraph::Graph<String, f32, petgraph::Directed>, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the output path includes the "output" directory
    let output_path = format!("output/{}", output_path);

    // Create the directory if it doesn't exist
    if let Some(parent) = std::path::Path::new(&output_path).parent() {
        fs::create_dir_all(parent)?;
    }

    // Write the graph in DOT format
    let dot = Dot::new(graph);
    let mut file = File::create(output_path.clone())?; // Clone output_path for reuse below
    writeln!(file, "{:?}", dot)?;
    
    println!("Graph exported to '{}'", output_path);

    Ok(())
}

