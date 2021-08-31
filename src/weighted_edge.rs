use std::fmt;

#[derive(Copy, Debug, Clone, PartialEq)] // pretty wild that rust can just derive these traits
pub struct WeightedEdge<T> {
	source: T, 
	sink: T,
	weight: u32,
}

impl<T: Copy> WeightedEdge<T> {
	pub fn new(source: T, sink: T, weight: u32) -> WeightedEdge<T> {
		WeightedEdge {
			source,
			sink,
			weight,
		}
	}

	pub fn get_source(&self) -> T {
		self.source
	}

	pub fn get_sink(&self) -> T {
		self.sink
	}

	pub fn get_weight(&self) -> u32 {
		self.weight
	}
}

impl<T: fmt::Debug> fmt::Display for WeightedEdge<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?} -- {:2} --> {:?}", self.source, self.weight, self.sink)
	}
}

#[cfg(test)]
mod test_weighted_edge {
	use super::*;

	#[test]
	fn test_format() {
		let edge = WeightedEdge::new(1, 2, 1);
		assert_eq!(format!("{}", edge), 
		format!("{:?} -- {:2} --> {:?}",
			edge.get_source(),
			edge.get_weight(),
			edge.get_sink()
		));

	}
}
