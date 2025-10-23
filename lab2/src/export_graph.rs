use std::collections::HashSet;
use std::fs::File;
use std::io::{Write, Result};

use crate::models::{Node, Rule};

pub fn export_to_dot(
    rules: &Vec<Rule>,
    filename: &str,
    target: Node,
    initial_nodes: Vec<Node>,
    closed_nodes: Vec<Node>,
    closed_rules: Vec<i32>
) -> Result<()> {
    let mut file = File::create(filename)?;

    let mut dot = String::new();
    dot.push_str("digraph RulesGraph {\n");
    dot.push_str("    node [fontname=\"Arial\"];\n");
    dot.push_str("    rankdir=BT;\n");

    for rule in rules {
        let color = if closed_rules.contains(&rule.id) {
            "lightgreen"
        } else {
            "lightgray"
        };
        dot.push_str(&format!(
            "    rule_{} [label=\"{}\", shape=box, style=filled, fillcolor={}];\n",
            rule.id, rule.id, color
        ));
    }

    let mut all_nodes = HashSet::new();
    for rule in rules {
        all_nodes.insert(rule.output.clone());
        for input in &rule.inputs {
            all_nodes.insert(input.clone());
        }
    }

    for node in &all_nodes {
        let mut color = "lightgray";
        if *node == target {
            color = "orange";
        } else if initial_nodes.contains(node) {
            color = "lightblue";
        } else if closed_nodes.contains(node) {
            color = "lightgreen";
        }

        dot.push_str(&format!(
            "    node_{} [label=\"{}\", shape=circle, style=filled, fillcolor={}];\n",
            node.value, node.value, color
        ));
    }

    for rule in rules {
        for input in &rule.inputs {
            dot.push_str(&format!("    node_{} -> rule_{};\n", input.value, rule.id));
        }
        dot.push_str(&format!(
            "    rule_{} -> node_{};\n",
            rule.id, rule.output.value
        ));
    }

    dot.push_str("}\n");

    writeln!(file, "{}", dot)?;
    Ok(())
}