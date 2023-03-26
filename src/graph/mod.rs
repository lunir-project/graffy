pub mod stable_graph;
pub mod unstable_graph;

/// Indicates that an acyclic graph will cycle if an edge is inserted.
#[derive(Debug)]
pub struct CycleError;

/// An uninhabited type that indicates that a graph's edges are directed.
#[derive(Clone, Debug)]
pub enum Directed {}
/// An uninhabited type that indicates that a graph's edges are undirected.
#[derive(Clone, Debug)]
pub enum Undirected {}

/// A trait for edge types.
pub trait EdgeType {
    /// Returns `true` if the edges of a graph are directed.
    fn is_directed(&self) -> bool;
}

impl EdgeType for Directed {
    /// Returns `true` if the edges of a graph are directed. Always returns `true`.
    fn is_directed(&self) -> bool {
        true
    }
}

impl EdgeType for Undirected {
    /// Returns true if the edge is directed, always returns `false`.
    fn is_directed(&self) -> bool {
        false
    }
}
