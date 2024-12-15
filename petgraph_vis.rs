use crate::data_processing::CleanRecord;
use petgraph::dot::{Dot, Config};
use petgraph::graph::UnGraph;
use petgraph::visit::EdgeRef;
use petgraph::algo::tarjan_scc;
use std::fs::{self, File};
use std::io::Write;

use std::process::Command;

pub fn export_graph_to_png(dot_file: &str, png_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("dot")
        .args(["-Tpng", dot_file, "-o", png_file])
        .status()?;
    
    if status.success() {
        println!("PNG file generated: {}", png_file);
    } else {
        eprintln!("Error converting DOT to PNG.");
    }

    Ok(())
}
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


pub fn cluster_states(graph: &UnGraph<String, f32>) -> Vec<Vec<String>> {
    tarjan_scc(graph)
        .into_iter()
        .map(|component| component.iter().map(|&node| graph[node].clone()).collect())
        .collect()
}

pub fn export_graph(graph: &petgraph::Graph<String, f32, petgraph::Directed>, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = std::path::Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }
    let dot = Dot::new(graph);
    let mut file = File::create(output_path)?;
    writeln!(file, "{:?}", dot)?;
    Ok(())
}


pub fn plot_graph_with_clusters(
    graph: &UnGraph<String, f32>,
    clusters: &[Vec<String>],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = std::path::Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut dot = String::new();
    dot.push_str("graph G {\n");

    // Add nodes with cluster-specific properties
    for (cluster_idx, cluster) in clusters.iter().enumerate() {
        let color = match cluster_idx % 6 {
            0 => "red",
            1 => "blue",
            2 => "green",
            3 => "yellow",
            4 => "purple",
            _ => "orange",
        };
        for node in cluster {
            dot.push_str(&format!(
                "  \"{}\" [color={} style=filled];\n",
                node, color
            ));
        }
    }

    // Add edges
    for edge in graph.edge_references() {
        dot.push_str(&format!(
            "  \"{}\" -- \"{}\";\n",
            graph[edge.source()],
            graph[edge.target()]
        ));
    }
    dot.push_str("}\n");

    fs::write(output_path, dot)?;
    Ok(())
}