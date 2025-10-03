#[derive(Debug, Clone)]
pub struct Node {
    pub number: usize,
    pub prev: Option<usize>,
}

impl Node {
    pub fn new(number: usize) -> Self {
        Node { number, prev: None }
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub start_node: usize,
    pub end_node: usize,
    pub label: i32,
    pub mark: i32,
}

impl Edge {
    pub fn new(start_node: usize, end_node: usize, label: i32) -> Self {
        Edge {
            start_node,
            end_node,
            label,
            mark: 0,
        }
    }
}