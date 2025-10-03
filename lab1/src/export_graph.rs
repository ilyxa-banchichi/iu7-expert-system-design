use std::fs::File;
use std::io::{Write, Result};

use crate::models::{Edge, Node};

pub fn export_to_dot(
    filename: &str,
    nodes: &Vec<Node>,
    edges: &Vec<Edge>,
    start: usize,
    goal: usize,
    path: Option<&[usize]>,
) -> Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "digraph G {{")?;
    writeln!(file, "    rankdir=LR;")?;
    writeln!(file, "    node [shape=circle];")?;

    for node in nodes {
        if node.number == start {
            writeln!(file, "    {} [style=filled, fillcolor=green];", node.number)?;
        } else if node.number == goal {
            writeln!(file, "    {} [style=filled, fillcolor=red];", node.number)?;
        } else {
            writeln!(file, "    {};", node.number)?;
        }
    }

    let mut path_edges: Vec<(usize, usize)> = vec![];
    if let Some(p) = path {
        for w in p.windows(2) {
            path_edges.push((w[0], w[1]));
        }
    }

    for edge in edges {
        if path_edges.contains(&(edge.start_node, edge.end_node)) {
            writeln!(
                file,
                "    {} -> {} [label=\"{}\", color=blue, penwidth=3.0];",
                edge.start_node, edge.end_node, edge.label
            )?;
        } else {
            writeln!(
                file,
                "    {} -> {} [label=\"{}\"];",
                edge.start_node, edge.end_node, edge.label
            )?;
        }
    }

    writeln!(file, "}}")?;
    Ok(())
}