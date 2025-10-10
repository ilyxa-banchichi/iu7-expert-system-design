use std::collections::{VecDeque, HashSet};

use crate::models::{Edge, Node};

pub struct GraphBFS {
    pub edge_list: Vec<Edge>,
    opened: VecDeque<usize>,
    closed: HashSet<usize>,
    start: Option<usize>,
    goal: Option<usize>,
    flag_yes: bool,
    flag_no: bool,
    child_counter: usize,
    nodes: Vec<Node>,
}

impl GraphBFS {
    pub fn new(edge_list: Vec<Edge>, node_count: usize) -> Self {
        GraphBFS {
            edge_list,
            opened: VecDeque::new(),
            closed: HashSet::new(),
            start: None,
            goal: None,
            flag_yes: true,
            flag_no: true,
            child_counter: 0,
            nodes: (0..node_count).map(Node::new).collect(),
        }
    }

    pub fn bfs(&mut self, start: usize, goal: usize) -> Vec<usize> {
        self.opened.push_back(start);
        self.start = Some(start);
        self.goal = Some(goal);

        while self.flag_yes && self.flag_no {
            self.search_descendants();

            // решение найдено
            if !self.flag_yes {
                if let Some(front) = self.opened.pop_front() {
                    self.closed.insert(front);
                }
                break;
            }

            if let Some(remove_node) = self.opened.pop_front() {
                if self.child_counter > 0 {
                    self.closed.insert(remove_node);
                } else if self.child_counter == 0 && self.opened.is_empty() {
                    self.flag_no = false;
                    println!("Нет решения");
                }
            }
        }

        return self.get_way();
    }

    fn search_descendants(&mut self) {
        self.child_counter = 0;

        if let (Some(&current), Some(goal)) = (self.opened.front(), self.goal.as_ref()) {
            if current == *goal {
                self.flag_yes = false;
                return;
            }

            for edge in self.edge_list.iter_mut() {
                if edge.start_node == current && edge.mark == 0 {
                    let next = edge.end_node;

                    // Пропускаем, если уже был в очереди или закрыт
                    if self.closed.contains(&next) || self.opened.contains(&next) {
                        continue;
                    }

                    // Если нашли цель — завершаем
                    if next == *goal {
                        self.nodes[next].prev = Some(current);
                        self.flag_yes = false;
                        return;
                    }

                    // Иначе добавляем потомка
                    self.nodes[next].prev = Some(current);
                    self.opened.push_back(next);
                    edge.mark = 1;
                    self.child_counter += 1;
                }
            }
        }
    }

    fn get_way(&self) -> Vec<usize> {
        let mut nodes = Vec::new();
        if let Some(goal) = self.goal {
            let mut temp = &self.nodes[goal];
            nodes.push(temp.number);
            while let Some(prev) = temp.prev {
                temp = &self.nodes[prev];
                nodes.push(temp.number);
            }
        }
        nodes.reverse();
        nodes
    }
}