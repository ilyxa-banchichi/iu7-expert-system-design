use crate::models::{Edge};

pub struct GraphDFS {
    pub edge_list: Vec<Edge>,
    pub opened_peak: Vec<usize>,
    closed_peak: Vec<usize>,
    goal: Option<usize>,
    j: i32,
    flag_yes: i32,
    flag_no: i32,
}

impl GraphDFS {
    pub fn new(edge_list: Vec<Edge>) -> Self {
        GraphDFS {
            edge_list,
            opened_peak: vec![],
            closed_peak: vec![],
            goal: None,
            j: 0,
            flag_yes: 1,
            flag_no: 1,
        }
    }

    pub fn dfs(&mut self, start: usize, goal: usize) -> Vec<usize> {
        self.opened_peak.push(start);
        self.goal = Some(goal);

        while self.flag_yes == 1 && self.flag_no == 1 {
            println!("{:?}", self.opened_peak.last());
            self.sample_search();
            if self.flag_yes == 0 {
                break;
            } else if self.j == 0 && !self.opened_peak.is_empty() {
                self.closed_peak.push(self.opened_peak.pop().unwrap());
            } else if self.j == 0 {
                println!("Решения нет");
                break;
            }
        }

        let mut path = self.opened_peak.clone();
        path.push(self.goal.unwrap());
        return path;
    }

    fn sample_search(&mut self) {
        self.j = 0;
        for edge in &mut self.edge_list {
            if let Some(&last) = self.opened_peak.last() {
                if edge.start_node == last && Some(edge.end_node) == self.goal {
                    self.j = 1;
                    self.flag_yes = 0;
                    break;
                } else if edge.start_node == last
                    && edge.mark == 0
                    && !self.closed_peak.contains(&edge.end_node)
                {
                    edge.mark = 1;
                    self.opened_peak.push(edge.end_node);
                    self.j = 1;
                    break;
                }
            }
        }
    }
}