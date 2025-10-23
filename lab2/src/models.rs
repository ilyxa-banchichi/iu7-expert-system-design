use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub value: i32,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub id: i32,
    pub inputs: Vec<Node>,
    pub output: Node,
    pub mark: bool
}

impl Rule {
    pub fn new(id: i32, inputs: Vec<Node>, output: Node) -> Self {
        Self {
            id,
            inputs,
            output,
            mark: false,
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}