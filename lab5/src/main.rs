use crate::{
    models::{Node, Rule, Variable},
    search::Search,
};

mod models;
mod search;

fn main() {
    let nodes = vec![
        // 0
        Node::new(
            "O".to_string(),
            vec![
                Variable::new_constant("N".to_owned()),
                Variable::new_constant("M1".to_owned()),
            ],
        ),
        // 1
        Node::new(
            "M".to_string(),
            vec![Variable::new_constant("M1".to_owned())],
        ),
        // 2
        Node::new(
            "A".to_string(),
            vec![Variable::new_constant("W".to_owned())],
        ),
        // 3
        Node::new(
            "E".to_string(),
            vec![
                Variable::new_constant("N".to_owned()),
                Variable::new_constant("A1".to_owned()),
            ],
        ),
        // 1 Rule
        // 4
        Node::new("W".to_string(), vec![Variable::new("y".to_owned())]),
        // 5
        Node::new("A".to_string(), vec![Variable::new("x".to_owned())]),
        // 6
        Node::new(
            "S".to_string(),
            vec![
                Variable::new("x".to_owned()),
                Variable::new("y".to_owned()),
                Variable::new("z".to_owned()),
            ],
        ),
        // 7
        Node::new("H".to_string(), vec![Variable::new("z".to_owned())]),
        // 8
        Node::new(
            "C".to_string(),
            vec![Variable::new_constant("x".to_owned())],
        ),
        // 2 Rule
        // 9
        Node::new("M".to_string(), vec![Variable::new("x1".to_owned())]),
        // 10
        Node::new(
            "O".to_string(),
            vec![
                Variable::new_constant("N".to_owned()),
                Variable::new_constant("x1".to_owned()),
            ],
        ),
        // 11
        Node::new(
            "S".to_string(),
            vec![
                Variable::new("W".to_owned()),
                Variable::new("x1".to_owned()),
                Variable::new("N".to_owned()),
            ],
        ),
        // 3 Rule
        // 12
        Node::new("M".to_string(), vec![Variable::new("x2".to_owned())]),
        // 13
        Node::new("W".to_string(), vec![Variable::new("x2".to_owned())]),
        // 4 Rule
        // 14
        Node::new(
            "E".to_string(),
            vec![
                Variable::new("x3".to_owned()),
                Variable::new("A1".to_owned()),
            ],
        ),
        // 15
        Node::new("H".to_string(), vec![Variable::new("x3".to_owned())]),
        // 16 Target
        Node::new("C".to_string(), vec![Variable::new("x0".to_owned())]),
    ];

    let rules = vec![
        Rule::new("r1".to_string(), 8, vec![4, 5, 6, 7]),
        Rule::new("r2".to_string(), 11, vec![9, 10]),
        Rule::new("r3".to_string(), 13, vec![12]),
        Rule::new("r4".to_string(), 15, vec![14]),
    ];

    let mut search = Search::new(rules, nodes, vec![0, 1, 2, 3], 16);
    search.search();
}
