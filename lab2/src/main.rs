mod export_graph;
mod models;

use crate::{export_graph::export_to_dot, models::{Node, Rule}};

pub struct GraphSearch {
    pub rules: Vec<Rule>,
    pub closed_nodes: Vec<Node>,
    pub target: Node,
    pub flag_y: bool,
    pub flag_n: bool,
}

impl GraphSearch {
    pub fn new(rules: Vec<Rule>, initial: Vec<Node>, target: Node)  -> Self {
        Self {
            rules,
            closed_nodes: initial,
            target,
            flag_y: true,
            flag_n: true
        }
    }

    pub fn get_closed_rules_id(&self) -> Vec<i32>{
        self.rules.iter().filter(|r| r.mark).map(|r| r.id).collect::<Vec<_>>()
    }

    pub fn bfs(&mut self) {
        print!("Список начальных вершин: ");
        for node in &self.closed_nodes {
            print!("{} ", node.value);
        }
        println!("\nЦелевая вершина: {}", self.target.value);

        while self.flag_y && self.flag_n {
            let count_closed_rules = self.pattern_search();
            if !self.flag_y {
                println!("Цель достигнута!");
                println!("Закрытые вершины: {}", self.closed_nodes.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "));
                println!("Закрытые правила: {}", self.get_closed_rules_id().iter().map(|r| r.to_string()).collect::<Vec<_>>().join(", "));
            } else if count_closed_rules == 0 {
                self.flag_n = false;
                println!("Нет решения");
            }
        }
    }

    fn pattern_search(&mut self) -> i32 {
        let mut count_closed_rules = 0;
        for rule in &mut self.rules {
            let mut count_closed_inputs = 0;
            for input in &rule.inputs {
                if self.closed_nodes.contains(&input) {
                    count_closed_inputs += 1;
                }
            }

            if !rule.mark && (rule.inputs.len() == count_closed_inputs) {
                println!("Закрываем правило {}", rule.id);
                rule.mark = true;
                self.closed_nodes.push(rule.output.clone());
                count_closed_rules += 1;
                if rule.output == self.target {
                    self.flag_y = false;
                }
            }
        }

        count_closed_rules
    }
}

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
    gs.bfs();

    export_to_dot(
        &rules,
        "output/result.dot",
        target.clone(),
        start_nodes.clone(),
        gs.closed_nodes.clone(),
        gs.get_closed_rules_id()
    ).unwrap();
}