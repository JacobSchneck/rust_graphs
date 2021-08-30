use crate::edge::Edge;

use std::collections::HashMap;
use std::collections::hash_set::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Debug;


pub struct Graph<T> {
	adjacency_list: HashMap<T, Vec<T>>,
}

impl<T: Debug> Graph<T> 
where T: std::cmp::Eq + std::hash::Hash + Copy + Clone {
	pub fn new() -> Graph<T> {
		Graph {adjacency_list: HashMap::new()}
	}

	pub fn get_adjacent_list<'a>(&'a self) -> &'a HashMap<T, Vec<T>> {
		&self.adjacency_list
	}

	pub fn add_edge(&mut self, edge: Edge<T>) {
		let sinks = self.adjacency_list.entry(edge.get_source()).or_insert(Vec::new());
		sinks.push(edge.get_sink());
	}

	pub fn bfs(&self, start: T) -> Vec<T> {
		let mut result = Vec::<T>::new();
		let mut queue = VecDeque::<T>::new();
		let mut set = HashSet::<T>::new();

		// if not in graph return empty vector (should I wrap in Option type?)
		if self.adjacency_list.get(&start).is_none() {
			return result;
		}

		queue.push_back(start);
		set.insert(start);

		while queue.len() != 0 {
			let popped = queue.pop_front().unwrap();
			result.push(popped);

			
			let adj_nodes = self.adjacency_list.get(&popped);
			match adj_nodes {
				Some(nodes) => {
					for node in nodes {
						if !set.contains(node) {
							queue.push_back(*node);
							set.insert(*node);
						}
					}
				},
				None => {},
			}
		}

		result
	}

	fn dfs_rec(&self, start: T, result: &mut Vec<T>, set: &mut HashSet<T>) {
		set.insert(start);
		result.push(start);
		match self.adjacency_list.get(&start) {
			Some(nodes) => {
				for node in nodes {
					if !set.contains(node) {
						self.dfs_rec(*node, result, set)
					}
				}
			}
			None => {}
		}
	}

	pub fn dfs(&self, start: T) -> Vec<T> {
		let mut result = Vec::<T>::new();
		let mut set = HashSet::<T>::new();

		if self.adjacency_list.get(&start).is_none() {
			return result;
		}

		self.dfs_rec(start, &mut result, &mut set);
		println!("{:?}", result);
		result
	}
}



impl<T: fmt::Debug> fmt::Display for Graph<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self.adjacency_list)
	}
}

#[cfg(test)] 
mod test {
	use super::*;

	#[test]
	fn add_edge() {
		let edge1 = Edge::new(1, 2);
		let edge2 = Edge::new(1, 3);
		let edge3 = Edge::new(2, 4);
		let edge4 = Edge::new(3, 4);

		let mut g = Graph::new();
		g.add_edge(edge1);
		g.add_edge(edge2);
		g.add_edge(edge3);
		g.add_edge(edge4);
		assert_eq!(g.get_adjacent_list().capacity(), 3);
		assert_eq!(g.get_adjacent_list().get_key_value(&1), Some((&1, &[2, 3].to_vec())));
	}

	#[test]
	fn bfs() {
		let edge1 = Edge::new(1, 2);
		let edge2 = Edge::new(1, 3);
		let edge3 = Edge::new(2, 4);
		let edge4 = Edge::new(3, 4);

		let mut g = Graph::new();
		g.add_edge(edge1);
		g.add_edge(edge2);
		g.add_edge(edge3);
		g.add_edge(edge4);
		assert_eq!(g.bfs(1), [1, 2, 3, 4]);
		assert_eq!(g.bfs(5), []);
	}

	#[test]
	fn dfs() {
		let edge1 = Edge::new(1, 2);
		let edge2 = Edge::new(1, 3);
		let edge3 = Edge::new(2, 4);
		let edge4 = Edge::new(3, 4);

		let mut g = Graph::new();
		g.add_edge(edge1);
		g.add_edge(edge2);
		g.add_edge(edge3);
		g.add_edge(edge4);
		assert_eq!(g.dfs(1), [1, 2, 4, 3]);
		assert_eq!(g.dfs(6), []);
	}

}