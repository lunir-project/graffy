mod unstable_graph;
mod stable_graph;

#[derive(Debug)]
pub struct CycleError;

#[derive(Clone, Debug)]
pub enum Directed {}
#[derive(Clone, Debug)]
pub enum Undirected {}

pub trait EdgeType {
    fn is_directed(&self) -> bool;
}

impl EdgeType for Directed {
    fn is_directed(&self) -> bool {
        true
    }
}

impl EdgeType for Undirected {
    fn is_directed(&self) -> bool {
        false
    }
}