use std::collections::VecDeque;

use crate::models::{Edge, Node};

pub struct GraphBFS {
    pub edge_list: Vec<Edge>,
    opened_peak: VecDeque<usize>,
    closed_peak: Vec<usize>,
    start: Option<usize>,
    goal: Option<usize>,
    flag_yes: i32,
    flag_no: i32,
    child_counter: usize,
}

impl GraphBFS {
    pub fn new(edge_list: Vec<Edge>) -> Self {
        GraphBFS {
            edge_list,
            opened_peak: VecDeque::new(),
            closed_peak: vec![],
            start: None,
            goal: None,
            flag_yes: 1,
            flag_no: 1,
            child_counter: 0,
        }
    }

    pub fn bfs(&mut self, nodes: &mut Vec<Node>, start: usize, goal: usize) -> Vec<usize> {
        self.opened_peak.push_back(start);
        self.start = Some(start);
        self.goal = Some(goal);

        while self.flag_yes == 1 && self.flag_no == 1 {
            self.search_descendants(nodes);
            if self.flag_yes == 0 {
                if let Some(node) = self.opened_peak.pop_front() {
                    self.closed_peak.push(node);
                }
                break;
            }

            if let Some(remove_node) = self.opened_peak.pop_front() {
                if self.child_counter > 0 {
                    self.closed_peak.push(remove_node);
                } else if self.child_counter == 0 && self.opened_peak.is_empty() {
                    self.flag_no = 0;
                    println!("Нет решения");
                }
            }
        }

        let mut result = vec![];
        result.push(self.goal.unwrap());
        if let Some(&last) = self.closed_peak.last() {
            let mut temp = Some(last);
            while let Some(i) = temp {
                result.push(i);
                temp = nodes[i].prev;
            }
            result.reverse();
        }

        result
    }

    fn search_descendants(&mut self, nodes: &mut Vec<Node>) {
        self.child_counter = 0;
        for edge in &mut self.edge_list {
            if let Some(&first) = self.opened_peak.front() {
                if Some(first) == self.goal {
                    self.flag_yes = 0;
                    break;
                } else if edge.start_node == first && edge.mark == 0 {
                    if edge.end_node == self.goal.unwrap() {
                        self.flag_yes = 0;
                        nodes[edge.end_node].prev = Some(first);
                        break;
                    } else {
                        nodes[edge.end_node].prev = Some(first);
                        self.opened_peak.push_back(edge.end_node);
                        edge.mark = 1;
                        self.child_counter += 1;
                    }
                }
            }
        }
    }
}