mod export_graph;
mod models;
mod bfs;
mod dfs;

use crate::bfs::GraphBFS;
use crate::dfs::GraphDFS;
use crate::models::{Edge, Node};
use crate::export_graph::export_to_dot;

fn main() {
    let nodes: Vec<Node> = (0..9).map(Node::new).collect();

    let edge_list = vec![
        Edge::new(0, 1, 10),
        Edge::new(0, 4, 11),
        Edge::new(0, 5, 13),

        Edge::new(1, 2, 12),

        Edge::new(2, 4, 14),
        Edge::new(4, 6, 15),
        Edge::new(4, 5, 16),

        Edge::new(5, 7, 17),

        Edge::new(3, 2, 18),

        Edge::new(7, 6, 23),
        Edge::new(7, 8, 20),

        Edge::new(6, 2, 19),
        Edge::new(6, 8, 21),

        Edge::new(8, 3, 22),
    ];

    let start = 0;
    let goal = 3;

    println!("Путь между {} и {}:", start, goal);

    let mut graph_dfs = GraphDFS::new(edge_list.clone());
    let dfs_path = graph_dfs.dfs(start, goal);
    println!("Путь в глубину {:?}", dfs_path);

    export_to_dot(
        "dfs.dot",
        &nodes,
        &graph_dfs.edge_list,
        start,
        goal,
        Some(&dfs_path),
    ).unwrap();

    // let mut nodes_bfs: Vec<Node> = (0..7).map(Node::new).collect();
    // let mut edge_list_bfs = edge_list.clone();
    // for e in &mut edge_list_bfs {
    //     e.mark = 0;
    // }

    // let mut graph_bfs = GraphBFS::new(edge_list_bfs);
    // let bfs_path = graph_bfs.bfs(&mut nodes_bfs, start, goal);
    // println!("Путь в ширину {:?}", bfs_path);

    // export_to_dot(
    //     "bfs.dot",
    //     &nodes_bfs,
    //     &graph_bfs.edge_list,
    //     0,
    //     4,
    //     Some(&bfs_path),
    // ).unwrap();
}