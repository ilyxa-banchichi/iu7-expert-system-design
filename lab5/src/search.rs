use std::collections::HashMap;

use crate::models::{Node, Rule, Variable};

pub struct Search {
    pub rules: Vec<Rule>,
    pub nodes: Vec<Node>,

    pub closed_nodes: Vec<usize>,
    pub closed_edges: Vec<usize>,
    pub open_nodes_stack: Vec<usize>,

    pub target: usize,
    pub flag_y: bool,
    pub flag_n: bool,
}

impl Search {
    pub fn new(
        rules: Vec<Rule>,
        nodes: Vec<Node>,
        closed_nodes: Vec<usize>,
        target: usize,
    ) -> Self {
        Self {
            rules,
            nodes,
            closed_nodes,
            closed_edges: Vec::new(),
            open_nodes_stack: vec![target],
            target,
            flag_y: true,
            flag_n: true,
        }
    }

    pub fn search(&mut self) {
        while self.flag_y && self.flag_n {
            let mut counter = 0;
            for (rule_idx, rule) in &mut self.rules.iter_mut().enumerate() {
                if rule.flag {
                    continue;
                }

                let current_node_idx = *self.open_nodes_stack.last().expect("");
                let current_node = &self.nodes[current_node_idx];
                let rule_final = &self.nodes[rule.final_node];

                let subst = Self::unification(current_node, rule_final);
                if subst.is_none() {
                    continue;
                }

                println!(
                    "_____________________________________________\nСтэк: {}",
                    self.open_nodes_stack
                        .iter()
                        .map(|arg| self.nodes[*arg].to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                println!(
                    "Доказано: {}",
                    self.closed_nodes
                        .iter()
                        .map(|arg| self.nodes[*arg].to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                println!(
                    "Унифицируем с правилом {} \"{}\"",
                    rule.name,
                    rule.print(&self.nodes)
                );

                for (key, value) in &subst.unwrap() {
                    if key.value != value.value {
                        println!("\t\tПодстановка \"{}\" в \"{}\"", value, key);
                    }
                    Self::substitution(&mut self.nodes[rule.final_node], key, value);
                    for inpt_idx in &rule.input_nodes {
                        Self::substitution(&mut self.nodes[*inpt_idx], key, value);
                    }
                    for stack_idx in &self.open_nodes_stack {
                        Self::substitution(&mut self.nodes[*stack_idx], key, value);
                    }
                }

                let mut temp = 0;
                for inpt_idx in rule.input_nodes.iter().rev() {
                    let mut subst2 = None;
                    for close_idx in &self.closed_nodes {
                        subst2 = Self::unification(&self.nodes[*inpt_idx], &self.nodes[*close_idx]);
                        if subst2.is_some() {
                            break;
                        }
                    }

                    if subst2.is_some() {
                        temp += 1;
                        println!("\tВершина {} доказана", self.nodes[*inpt_idx]);
                        for (key, value) in &subst2.unwrap() {
                            if key.value != value.value {
                                println!("\t\tПодстановка \"{}\" в \"{}\"", value, key);
                            }
                            Self::substitution(&mut self.nodes[rule.final_node], key, value);
                            for inpt_idx in &rule.input_nodes {
                                Self::substitution(&mut self.nodes[*inpt_idx], key, value);
                            }
                            for stack_idx in &self.open_nodes_stack {
                                Self::substitution(&mut self.nodes[*stack_idx], key, value);
                            }
                        }
                    } else {
                        println!("\tВершина {} ещё не доказана ", self.nodes[*inpt_idx]);
                        self.open_nodes_stack.push(*inpt_idx);
                    }
                }

                if temp == rule.input_nodes.len() {
                    println!("Доказали правило");
                    rule.flag = true;
                    self.closed_nodes.push(rule.final_node);
                    self.closed_edges.push(rule_idx);

                    if self.target == current_node_idx {
                        self.flag_y = false;
                    }

                    self.open_nodes_stack.pop();
                    counter += 1;
                } else {
                    println!("Правило пока не доказано");
                }
            }

            if counter == 0 {
                self.flag_n = false;
                println!("Рещения нет");
            }

            if !self.flag_y {
                println!("Рещения найдено");
            }
        }
    }

    fn substitution(n: &mut Node, name: &Variable, value: &Variable) {
        for i in 0..n.args.len() {
            if n.args[i].name == *name.name {
                n.args[i] = value.clone()
            }
        }
    }

    fn unification(n1: &Node, n2: &Node) -> Option<HashMap<Variable, Variable>> {
        let is_same_name = n1.name == n2.name;
        let is_same_args_count = n1.args.len() == n2.args.len();

        if !is_same_name || !is_same_args_count {
            return None;
        }

        let mut subst = HashMap::new();

        for arg_idx in 0..n1.args.len() {
            let arg1 = &n1.args[arg_idx];
            let arg2 = &n2.args[arg_idx];

            // Константа - Константа
            if arg1.is_constant && arg2.is_constant {
                if arg1.value != arg2.value {
                    return None;
                }

                subst.insert(arg1.clone(), arg2.clone());
                continue;
            }

            // Переменная - переменная
            if !arg1.is_constant && !arg2.is_constant {
                if arg1.has_value() && !arg2.has_value() {
                    subst.insert(arg2.clone(), arg1.clone());
                } else {
                    subst.insert(arg1.clone(), arg2.clone());
                }

                continue;
            }

            if !arg1.is_constant && arg2.is_constant {
                subst.insert(arg1.clone(), arg2.clone());
            } else {
                subst.insert(arg2.clone(), arg1.clone());
            }
        }

        if subst.len() == 0 {
            return None;
        }

        return Some(subst);
    }
}
