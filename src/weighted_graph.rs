use crate::weighted_edge::WeightedEdge;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use priority_queue::PriorityQueue;
use core::cmp::max;

use std::fmt;

// Consider anding parent that tracks the path taken? Could also just use hashmap
pub struct Node<T> {
	id: T,
	weight: u32
}

impl<T> Node<T> {
	pub fn new(id: T, weight: u32) -> Self {
		Node {
			id,
			weight,
		}
	}
}

impl<T: fmt::Debug> fmt::Display for Node<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?} : {}", self.id, self.weight)
	}
}

pub struct WeightedGraph<T> {
	adjacency_list: HashMap<T, Vec<WeightedEdge::<T>>>,
}

impl<T> WeightedGraph<T> 
where T: std::cmp::Eq + std::hash::Hash + Copy + std::fmt::Debug {
	pub fn new() -> WeightedGraph<T> {
		WeightedGraph {
			adjacency_list: HashMap::new()
		}
	}

	pub fn get_adjacent_list<'a>(&'a self) -> &'a HashMap<T, Vec<WeightedEdge::<T>>> {
		&self.adjacency_list
	}

	pub fn add_edge(&mut self, edge: WeightedEdge<T>) {
		let sinks = self.adjacency_list.entry(edge.get_source()).or_insert(Vec::new());
		sinks.push(edge);
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
						if !set.contains(&node.get_sink()) {
							queue.push_back(node.get_sink());
							set.insert(node.get_sink());
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
					if !set.contains(&node.get_sink()) {
						self.dfs_rec(node.get_sink(), result, set)
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
		// println!("{:?}", result);
		result
	}

	// Made a reverse dijstrka's algorithm (longes path to each node) for lulz
	pub fn reverse_dijkstra(&self, start: T) -> HashMap<T, u32> {
		let mut result = HashMap::new();
		if self.adjacency_list.get(&start).is_none() {
			return result;
		}

		let mut queue = PriorityQueue::<T, u32>::new();
		queue.push(start, 0);

		while queue.len() != 0 {
			let (node, weight) = queue.pop().unwrap();

			let node_weight = result.entry(node).or_insert(weight);
			*node_weight = weight;

			if let Some(adj_edges) = self.adjacency_list.get(&node) {
				for edge in adj_edges {
					match queue.get(&edge.get_sink()) {
						Some(_) => {
							let old_priority = queue.get_priority(&edge.get_sink()).unwrap();
							let new_priority = edge.get_weight() + weight;
							queue.change_priority(&edge.get_sink(), max(*old_priority, new_priority)); 

						}
						None => {
							queue.push(edge.get_sink(), edge.get_weight() + weight);
						}
					}
				}
			}
		}

		result
		// println!("len {}", result.len());
	}
}

impl<T: fmt::Debug + Eq + std::hash::Hash> fmt::Display for WeightedGraph<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// Probably would give a Senior Engineer an migraine 
		Ok(for key in self.adjacency_list.keys() {
			write!(f, "{:?}:\n", key);	
			let vals = self.adjacency_list.get(key).unwrap();
			for val in vals {
				write!(f, "{}\n", val);
			}
			write!(f, "\n");
		})
		// write!(f, "{:#?}", self.adjacency_list)
	}
}


#[cfg(test)]
mod test_weighted_graph {
	use super::*;

	#[test]
	fn test_add_edge() {
		let edge1 = WeightedEdge::new(1, 2, 1);
		let edge2 = WeightedEdge::new(1, 3, 1);
		let edge3 = WeightedEdge::new(2, 4, 1);
		let edge4 = WeightedEdge::new(3, 4, 1);

		let mut g = WeightedGraph::new();
		g.add_edge(edge1);
		g.add_edge(edge2);
		g.add_edge(edge3);
		g.add_edge(edge4);

		assert_eq!(g.get_adjacent_list().capacity(), 3);
		assert_eq!(g.get_adjacent_list().get_key_value(&1), Some((&1, &[edge1, edge2].to_vec())));
	}

	#[test]
	fn bfs() {
		let edge1 = WeightedEdge::new(1, 2, 1);
		let edge2 = WeightedEdge::new(1, 3, 1);
		let edge3 = WeightedEdge::new(2, 4, 1);
		let edge4 = WeightedEdge::new(3, 4, 1);

		let mut g = WeightedGraph::new();
		g.add_edge(edge1);
		g.add_edge(edge2);
		g.add_edge(edge3);
		g.add_edge(edge4);
		assert_eq!(g.bfs(1), [1, 2, 3, 4]);
		assert_eq!(g.bfs(5), []);
	}

	#[test]
	fn dfs() {
		let edge1 = WeightedEdge::new(1, 2, 1);
		let edge2 = WeightedEdge::new(1, 3, 1);
		let edge3 = WeightedEdge::new(2, 4, 1);
		let edge4 = WeightedEdge::new(3, 4, 1);

		let mut g = WeightedGraph::new();
		g.add_edge(edge1);
		g.add_edge(edge2);
		g.add_edge(edge3);
		g.add_edge(edge4);
		assert_eq!(g.dfs(1), [1, 2, 4, 3]);
		assert_eq!(g.dfs(6), []);
	}

	#[test]
	fn reverse_dijkstra() {
		let edge1 = WeightedEdge::new(1, 2, 1);
		let edge2 = WeightedEdge::new(1, 3, 1);
		let edge3 = WeightedEdge::new(2, 4, 1);
		let edge4 = WeightedEdge::new(3, 4, 1);

		let mut g = WeightedGraph::new();
		g.add_edge(edge1);
		g.add_edge(edge2);
		g.add_edge(edge3);
		g.add_edge(edge4);
		g.reverse_dijkstra(1);
		let mut rhs = HashMap::<i32, u32>::new();
		rhs.insert(1, 0);
		rhs.insert(2, 1);
		rhs.insert(3, 1);
		rhs.insert(4, 2);
		assert_eq!(g.reverse_dijkstra(1), rhs);
		rhs.clear();
		assert_eq!(g.reverse_dijkstra(4), rhs);
		assert_eq!(g.reverse_dijkstra(6), rhs);
		rhs.insert(3, 0);
		rhs.insert(4, 1);
		assert_eq!(g.reverse_dijkstra(3), rhs);
	}
}