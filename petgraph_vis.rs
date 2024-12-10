use petgraph::Graph;
use petgraph::dot::{Dot, Config};
//use crate::data_processing::CleanRecord;
use std::fs::{self, File};
use std::io::Write;


pub fn export_graph(graph: &Graph<String, f32>, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = std::path::Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }
    let dot = Dot::with_config(graph, &[Config::EdgeNoLabel]);
    let mut file = File::create(output_path)?;
    writeln!(file, "{:?}", dot)?;
    Ok(())
}


