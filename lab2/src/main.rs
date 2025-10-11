use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    flag: i32,
}

impl Node {
    fn new(value: i32) -> Self {
        Self { value, flag: 0 }
    }
}

#[derive(Debug)]
struct Edge {
    value: i32,
    final_node: Rc<RefCell<Node>>,
    input_nodes: Vec<Rc<RefCell<Node>>>,
    label: i32,
}

impl Edge {
    fn new(value: i32, final_node: Rc<RefCell<Node>>, input_nodes: Vec<Rc<RefCell<Node>>>) -> Self {
        Self {
            value,
            final_node,
            input_nodes,
            label: 0,
        }
    }
}

struct GraphSearch {
    edge_list: Vec<Edge>,
    closed_nodes: Vec<Rc<RefCell<Node>>>,
    target_node: Rc<RefCell<Node>>,
    flag_y: bool,
    flag_n: bool,
    open_edges: Vec<Edge>,
}

impl GraphSearch {
    fn new(
        edge_list: Vec<Edge>,
        target_node: Rc<RefCell<Node>>,
        default_nodes: Vec<Rc<RefCell<Node>>>,
    ) -> Self {
        Self {
            edge_list,
            closed_nodes: default_nodes,
            target_node,
            flag_y: true,
            flag_n: true,
            open_edges: Vec::new(),
        }
    }

    fn bfs(&mut self) {
        println!("Список начальных вершин:");
        for node in &self.closed_nodes {
            print!("{} ", node.borrow().value);
        }
        println!("\nЦелевая вершина: {}", self.target_node.borrow().value);

        while self.flag_y && self.flag_n {
            let count = self.pattern_search();
            if !self.flag_y {
                println!("Решение найдено");
                break;
            } else if count == 0 {
                self.flag_n = false;
                println!("Нет решения");
            }
        }
    }

    fn pattern_search(&mut self) -> usize {
        let mut count = 0;
        for edge in &mut self.edge_list {
            if edge.label == 0 {
                let mut temp = 0;
                for input_node in &edge.input_nodes {
                    if self
                        .closed_nodes
                        .iter()
                        .any(|n| Rc::ptr_eq(n, input_node))
                    {
                        input_node.borrow_mut().flag = 1;
                        temp += 1;
                    }
                }

                if temp == edge.input_nodes.len() {
                    count += 1;
                    edge.label = 1;
                    edge.final_node.borrow_mut().flag = 1;
                    self.closed_nodes.push(Rc::clone(&edge.final_node));
                    self.open_edges.push(edge.clone());

                    if Rc::ptr_eq(&edge.final_node, &self.target_node) {
                        self.flag_y = false;
                        break;
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let node1 = Rc::new(RefCell::new(Node::new(1)));
    let node2 = Rc::new(RefCell::new(Node::new(2)));
    let node3 = Rc::new(RefCell::new(Node::new(3)));
    let node4 = Rc::new(RefCell::new(Node::new(4)));
    let node5 = Rc::new(RefCell::new(Node::new(5)));
    let node6 = Rc::new(RefCell::new(Node::new(6)));
    let node7 = Rc::new(RefCell::new(Node::new(7)));
    let node8 = Rc::new(RefCell::new(Node::new(8)));
    let node9 = Rc::new(RefCell::new(Node::new(9)));
    let node10 = Rc::new(RefCell::new(Node::new(10)));
    let node11 = Rc::new(RefCell::new(Node::new(11)));
    let node12 = Rc::new(RefCell::new(Node::new(12)));
    let node13 = Rc::new(RefCell::new(Node::new(13)));
    let node14 = Rc::new(RefCell::new(Node::new(14)));
    let node15 = Rc::new(RefCell::new(Node::new(15)));
    let node16 = Rc::new(RefCell::new(Node::new(16)));
    let node17 = Rc::new(RefCell::new(Node::new(17)));
    let node18 = Rc::new(RefCell::new(Node::new(18)));
    let node19 = Rc::new(RefCell::new(Node::new(19)));
    let node20 = Rc::new(RefCell::new(Node::new(20)));
    let node31 = Rc::new(RefCell::new(Node::new(31)));
    let node33 = Rc::new(RefCell::new(Node::new(33)));

    let edges = vec![
        Edge::new(104, Rc::clone(&node3), vec![Rc::clone(&node8), Rc::clone(&node31)]),
        Edge::new(101, Rc::clone(&node3), vec![Rc::clone(&node1), Rc::clone(&node2)]),
        Edge::new(107, Rc::clone(&node7), vec![Rc::clone(&node3), Rc::clone(&node2), Rc::clone(&node4)]),
        Edge::new(103, Rc::clone(&node4), vec![Rc::clone(&node5), Rc::clone(&node6)]),
        Edge::new(107, Rc::clone(&node11), vec![Rc::clone(&node12), Rc::clone(&node13)]),
        Edge::new(106, Rc::clone(&node9), vec![Rc::clone(&node4), Rc::clone(&node10), Rc::clone(&node11)]),
        Edge::new(105, Rc::clone(&node14), vec![Rc::clone(&node7), Rc::clone(&node9)]),
        Edge::new(110, Rc::clone(&node14), vec![Rc::clone(&node9), Rc::clone(&node18)]),
        Edge::new(112, Rc::clone(&node18), vec![Rc::clone(&node19), Rc::clone(&node20)]),
        Edge::new(109, Rc::clone(&node15), vec![Rc::clone(&node16), Rc::clone(&node17)]),
        Edge::new(108, Rc::clone(&node33), vec![Rc::clone(&node15), Rc::clone(&node18)]),
    ];

    let start_nodes = vec![
        Rc::clone(&node19),
        Rc::clone(&node20),
        Rc::clone(&node12),
        Rc::clone(&node13),
        Rc::clone(&node10),
        Rc::clone(&node5),
        Rc::clone(&node6),
    ];

    let mut graph_search = GraphSearch::new(edges, Rc::clone(&node14), start_nodes);

    graph_search.bfs();
}