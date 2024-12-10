
use petgraph::Graph;
use petgraph::algo::dijkstra;
//use plotly::{Bar, Plot};
use crate::data_processing::CleanRecord;
use petgraph::prelude::*;
use std::collections::HashMap;
//use plotters::prelude::*;
//use plotters::style::Color; // For `BLUE` and `WHITE`
//use plotters::style::colors::WHITE; // Sometimes needs explicit import
//use plotters::coord::types::RangedCoordusize; // To handle integer axes
//use std::error::Error;


pub fn construct_graph(records: &[CleanRecord]) -> Graph<String, f32> {
    let mut graph = Graph::<String, f32>::new();
    let mut state_indices = std::collections::HashMap::new();

    for record in records {
        if !state_indices.contains_key(&record.jurisdiction) {
            let node_index = graph.add_node(record.jurisdiction.clone());
            state_indices.insert(record.jurisdiction.clone(), node_index);
        }
    }

    for i in 0..records.len() {
        for j in i + 1..records.len() {
            let state1 = &records[i];
            let state2 = &records[j];
            let rate_diff = (state1.incarceration_rate - state2.incarceration_rate).abs();

            if rate_diff < 50.0 {
                let idx1 = state_indices[&state1.jurisdiction];
                let idx2 = state_indices[&state2.jurisdiction];
                graph.add_edge(idx1, idx2, rate_diff);
            }
        }
    }

    graph
}

pub fn compute_degree_centrality(graph: &Graph<String, f32>) -> Vec<(String, usize)> {
    let result: Vec<(String, usize)> = graph.node_indices()
        .map(|node| {
            let state = graph[node].clone();
            let degree = graph.edges(node).count();
            (state, degree)
        })
        .collect();

    if result.is_empty() {
        eprintln!("No nodes with degree > 0.");
    }

    result
}
pub fn compute_k_core(graph: &Graph<String, f32>, k: usize) -> Vec<String> {
    graph.node_indices()
        .filter(|node| graph.edges(*node).count() >= k)
        .map(|node| graph[node].clone())
        .collect()
}

pub fn compute_average_shortest_path(graph: &Graph<String, f32, Directed>) -> f32 {
    let mut total_distance = 0.0;
    let mut path_count = 0;

    for start in graph.node_indices() {
        // Run Dijkstra's algorithm from each node
        let distances: HashMap<NodeIndex, f32> = dijkstra(graph, start, None, |edge| *edge.weight());

        // Sum up distances to reachable nodes
        for (_, distance) in distances.iter() {
            total_distance += *distance;
            path_count += 1;
        }
    }

    // Calculate average shortest path length
    if path_count == 0 {
        return 0.0; // Avoid division by zero
    }

    total_distance / path_count as f32
}

pub fn group_states_by_centrality(centrality: &[(String, usize)]) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut high = vec![];
    let mut medium = vec![];
    let mut low = vec![];

    for (state, degree) in centrality {
        if *degree > 1000 {
            high.push(state.clone());
        } else if *degree > 500 {
            medium.push(state.clone());
        } else {
            low.push(state.clone());
        }
    }

    (high, medium, low)
}