// use graphs::edge::Edge;

use graphs::edge::Edge;
use graphs::graph::Graph;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use regex::Regex;
// use std::fs;

fn main() {
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
    let mut g3 = Graph::new();
    let mut g4 = Graph::new();

    // read from edges
    let args: Vec<String> = env::args().collect();
    let filename = &args[2];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(edge) = line {
                // let edge_data_str = regex::Regex::new(r"[, ]+").unwrap()
                // .split(&edge)
                // .map(|node| node.to_string())
                // .collect::<Vec<String>>();

                // fantastic line of code 
                let edge_data_u_int = regex::Regex::new(r"[, ]+").unwrap()
                .split(&edge)
                .map(|node| node.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

                let edge_data_char = regex::Regex::new(r"[, ]+").unwrap()
                .split(&edge)
                .map(|node| node.parse::<char>().unwrap())
                .collect::<Vec<char>>();

                g2.add_edge(Edge::new(edge_data_u_int[0], edge_data_u_int[1]));
                g3.add_edge(Edge::new(edge_data_char[0], edge_data_char[1]));
                // g4.add_edge(Edge::new(&edge_data_str[0], &edge_data_str[1]));
            }
        }
    }

    println!("Unsigned int graph:\n{}\n", g2);
    println!("Character graph:\n{}\n", g3);
    println!("Character graph:\n{}\n", g4);

    // let contents = fs::read_to_string(filename).expect("Probably Error 404");

    // println!("{}", contents);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Error 404");
    Ok(io::BufReader::new(file).lines())
}