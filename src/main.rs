// use graphs::edge::Edge;

use graphs::edge::Edge;
use graphs::graph::Graph;
use graphs::weighted_edge::WeightedEdge;
use graphs::weighted_graph::WeightedGraph;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[2];
    // println!("{:?}", args);
    if args.len() > 3 {
        run_weighted_graph(filename);
    } else {
        run_graph(filename);
    }

}

fn run_graph(filename: &String) {
    let edge: Edge<i32> = Edge::new(1, 2);
    println!("{}", edge);

    let edge1 = Edge::new(1, 2);
    let edge2 = Edge::new(1, 3);
    let edge3 = Edge::new(2, 4);
    let edge4 = Edge::new(3, 4);

    let mut g = Graph::new();
    g.add_edge(edge1);
    g.add_edge(edge2);
    g.add_edge(edge3);
    g.add_edge(edge4);

    println!("{}", g);
    println!("{:?}", g.bfs(1));
    println!("{:?}", g.bfs(5));

    let mut g2 = Graph::<u32>::new();

    // read from edges
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(edge) = line {
                // fantastic line of code 
                let edge_data_u_int = regex::Regex::new(r"[, ]+").unwrap()
                .split(&edge)
                .map(|node| node.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

                g2.add_edge(Edge::new(edge_data_u_int[0], edge_data_u_int[1]));
            }
        }
    }

    println!("Unsigned int graph:\n{}\n", g2);

}

fn run_weighted_graph(filename: &String) {
    let mut g2 = WeightedGraph::<u32>::new();

    // read from edges
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(edge) = line {
                // fantastic line of code 
                let edge_data_u_int = regex::Regex::new(r"[, ]+").unwrap()
                .split(&edge)
                .map(|node| node.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
                let edge = WeightedEdge::new(edge_data_u_int[0], edge_data_u_int[1], edge_data_u_int[2]);
                g2.add_edge(edge);
            }
        }
    }

    println!("Unsigned int graph:\n{}", g2);
    println!("Dijkstra's algorithm : {:?}", g2.reverse_dijkstra(0));

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Error 404");
    Ok(io::BufReader::new(file).lines())
}