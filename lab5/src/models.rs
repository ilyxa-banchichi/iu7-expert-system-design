use std::fmt;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub args: Vec<Variable>,
}

impl Node {
    pub fn new(name: String, args: Vec<Variable>) -> Self {
        Self { name, args }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_str = self
            .args
            .iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}({})", self.name, args_str)
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub final_node: usize,
    pub input_nodes: Vec<usize>,
    pub flag: bool,
}

impl Rule {
    pub fn new(name: String, final_node: usize, input_nodes: Vec<usize>) -> Self {
        Self {
            name,
            final_node,
            input_nodes,
            flag: false,
        }
    }

    pub fn print(&self, nodes: &Vec<Node>) -> String {
        let inp_str = self
            .input_nodes
            .iter()
            .map(|arg| nodes[*arg].to_string())
            .collect::<Vec<_>>()
            .join(", ");
        return format!("{} -> {}", inp_str, nodes[self.final_node]);
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Variable {
    pub is_constant: bool,
    pub name: String,
    pub value: Option<String>,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Self {
            is_constant: false,
            name,
            value: None,
        }
    }

    pub fn new_constant(value: String) -> Self {
        Self {
            is_constant: true,
            name: value.clone(),
            value: Some(value),
        }
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &_ = if self.has_value() {
            &self.value.clone().unwrap()
        } else {
            &self.name
        };
        write!(f, "{}", s)
    }
}
