mod export_graph;
mod models;
mod graph_search;

use crate::{export_graph::export_to_dot, graph_search::GraphSearch, models::{Node, Rule}};

fn main() {
    let rules = vec![
        Rule::new(104, vec![Node::new(8), Node::new(21)], Node::new(3)),
        Rule::new(101, vec![Node::new(1), Node::new(2)], Node::new(3)),
        Rule::new(102, vec![Node::new(3), Node::new(2), Node::new(4)], Node::new(7)),
        Rule::new(103, vec![Node::new(5), Node::new(6)], Node::new(4)),
        Rule::new(107, vec![Node::new(12), Node::new(13)], Node::new(11)),
        Rule::new(106, vec![Node::new(4), Node::new(10), Node::new(11)], Node::new(9)),
        Rule::new(105, vec![Node::new(7), Node::new(9)], Node::new(14)),
        Rule::new(110, vec![Node::new(9), Node::new(18)], Node::new(14)),
        Rule::new(112, vec![Node::new(19), Node::new(20)], Node::new(18)),
        Rule::new(109, vec![Node::new(16), Node::new(17)], Node::new(15)),
        Rule::new(108, vec![Node::new(15), Node::new(18)], Node::new(22)),
    ];

    let start_nodes = vec![
        Node::new(19),
        Node::new(20),
        Node::new(12),
        Node::new(13),
        Node::new(10),
        Node::new(5),
        Node::new(6),
    ];

    let target = Node::new(14);

    let mut gs: GraphSearch = GraphSearch::new(rules.clone(), start_nodes.clone(), target.clone());
    gs.search();

    export_to_dot(
        &rules,
        "output/result.dot",
        target.clone(),
        start_nodes.clone(),
        gs.closed_nodes.clone(),
        gs.get_closed_rules_id()
    ).unwrap();
}