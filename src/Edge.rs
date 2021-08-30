use std::fmt;

// #[derive(Copy, Clone)]
pub struct Edge<T> {
	source: T,
	sink: T,
}

impl<T: Copy> Edge<T> {
	pub fn new(source: T, sink: T) -> Edge<T> {
		Edge {source: source, sink: sink}
	}

	pub fn get_source(&self) -> T {
		self.source
	}

	pub fn get_sink(&self) -> T {
		self.sink
	}
}

impl<T: fmt::Debug> fmt::Display for Edge<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?} -> {:?}", self.source, self.sink)
	}
}

#[cfg(test)]
mod test {

}