mod tests;

use std::marker::PhantomData;

#[derive(Debug)]
pub enum Cyclic {}
#[derive(Debug)]
pub enum Acyclic {}

pub trait Cyclicity {
    fn is_cyclic(&self) -> bool;
}

impl Cyclicity for Cyclic {
    fn is_cyclic(&self) -> bool {
        true
    }
}

impl Cyclicity for Acyclic {
    fn is_cyclic(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub struct Vertex<V, E> {
    pub weight: V,
    pub edges: Vec<Edge<E, V>>,
}

#[derive(Debug)]
pub enum Directed {}
#[derive(Debug)]
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

pub enum Direction {
    Outgoing,
    Incoming,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Outgoing => Self::Incoming,
            Self::Incoming => Self::Outgoing,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Edge<E, V> {
    pub weight: E,
    pub vertices: [Vertex<E, V>; 2],
}

#[derive(Clone, Debug)]
pub struct Graph<V, E, D: EdgeType, C: Cyclicity> {
    vertices: Vec<Vertex<V, E>>,
    edges: Vec<Edge<E, V>>,
    directed: PhantomData<D>,
    cyclic: PhantomData<C>,
}

impl<E, V> Edge<E, V> {
    pub fn new(weight: E, source: Vertex<E, V>, destination: Vertex<E, V>) -> Edge<E, V> {
        Edge {
            weight,
            vertices: [source, destination],
        }
    }

    pub fn vertices(&self) -> (&Vertex<E, V>, &Vertex<E, V>) {
        (&self.vertices[0], &self.vertices[1])
    }
}

impl<V, E> Vertex<V, E> {
    pub fn new(weight: V) -> Vertex<V, E> {
        Vertex {
            weight,
            edges: vec![],
        }
    }
}

impl<V, E, C: Cyclicity> Graph<V, E, Directed, C> {}

impl<V, E, C: Cyclicity> Graph<V, E, Undirected, C> {}

impl<V, E, D: EdgeType> Graph<V, E, D, Cyclic> {
    pub fn add_edge(&mut self, source: Vertex<E, V>, destination: Vertex<E, V>, weight: E) {
        self.edges.push(Edge::new(weight, source, destination));
    }

    pub fn add_vertex(&mut self, weight: V) {
        self.vertices.push(Vertex::new(weight));
    }
}

impl<V, E, D: EdgeType> Graph<V, E, D, Acyclic> {}

impl<V, E, D: EdgeType, C: Cyclicity> Graph<V, E, D, C> {
    pub fn with_capacity(cap: usize) -> Graph<V, E, D, C> {
        Graph {
            vertices: Vec::with_capacity(cap),
            edges: Vec::with_capacity(cap),
            directed: PhantomData,
            cyclic: PhantomData,
        }
    }

    pub fn new_undirected() -> Graph<V, E, Undirected, C> {
        Graph {
            vertices: vec![],
            edges: vec![],
            directed: PhantomData,
            cyclic: PhantomData,
        }
    }

    pub fn new_directed() -> Graph<V, E, Directed, C> {
        Graph {
            vertices: vec![],
            edges: vec![],
            directed: PhantomData,
            cyclic: PhantomData,
        }
    }
}
