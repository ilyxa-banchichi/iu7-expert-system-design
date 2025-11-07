use std::fmt;

#[derive(Debug, Clone, Hash)]
pub struct Node {
    pub value: i32,
    pub flag: bool // 0 - не доказана, 1 - доказана
}

impl Node {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            flag: false
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Node {}

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
    pub count: i32,
    pub mark: i32
}

impl Rule {
    pub fn new(id: i32, inputs: Vec<Node>, output: Node) -> Self {
        Self {
            id,
            inputs,
            output,
            count: 0,
            mark: 0,
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}