mod tests;

use std::{collections::HashSet, iter::Cycle, marker::PhantomData};

#[derive(Debug)]
pub enum Cyclic {}
#[derive(Debug)]
pub enum Acyclic {}

pub struct CycleError;

pub trait Cyclicness: Sized {
    fn insert_edge<V: Clone, E: Clone, D: EdgeType>(
        graph: &mut Graph<V, E, D, Self>,
        edge: Edge<E, V>,
    ) -> Result<usize, CycleError>;
}

impl Cyclicness for Cyclic {
    fn insert_edge<V: Clone, E: Clone, D: EdgeType>(
        graph: &mut Graph<V, E, D, Self>,
        edge: Edge<E, V>,
    ) -> Result<usize, CycleError> {
        Ok(graph.insert_edge_unchecked(edge))
    }
}

impl Cyclicness for Acyclic {
    fn insert_edge<V: Clone, E: Clone, D: EdgeType>(
        graph: &mut Graph<V, E, D, Self>,
        edge: Edge<E, V>,
    ) -> Result<usize, CycleError> {
        if graph.would_cycle_with_edge(edge.clone()) {
            return Err(CycleError);
        }

        Ok(graph.insert_edge_unchecked(edge))
    }
}

#[derive(Clone, Debug)]
pub struct Vertex<V: Clone> {
    pub weight: V,
    pub edges: Vec<usize>,
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
pub struct Edge<E: Clone, V> {
    pub weight: E,
    pub vertices: [usize; 2],
    _phantom: PhantomData<V>,
}

#[derive(Clone, Debug)]
pub struct Graph<V: Clone, E: Clone, D: EdgeType, C: Cyclicness> {
    vertices: Vec<Vertex<V>>,
    edges: Vec<Edge<E, V>>,
    directed: PhantomData<D>,
    cyclic: PhantomData<C>,
}

impl<V: Clone, E: Clone, D: EdgeType, C: Cyclicness> Graph<V, E, D, C> {
    pub fn cycles(&self) -> bool {
        todo!();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut idx = 0;

        loop {
            for edge in self.vertices[idx].edges {
                let target = &self.edges[edge].vertices[0];

                visited.insert(*target);

                if visited.get(target).is_some() {
                    return true;
                }
            }
        }

        false
    }

    pub fn insert_edge(
        &mut self,
        source: usize,
        destination: usize,
        weight: E,
    ) -> Result<usize, CycleError> {
        C::insert_edge(self, Edge::new(source, destination, weight))
    }

    fn insert_edge_unchecked(&mut self, edge: Edge<E, V>) -> usize {
        let index = self.edges.len();
        let (from, to) = edge.vertex_indices();

        match self.vertices.get_mut(from) {
            Some(source_vertex) => {
                source_vertex.associate_edge(index);
            }

            _ => panic!("Invalid index in edge upon insertion into graph."),
        };

        match self.vertices.get_mut(to) {
            Some(target_vertex) => {
                target_vertex.associate_edge(index);
            }

            _ => panic!("Invalid index in edge upon insertion into graph."),
        };

        self.edges.push(edge);

        index
    }

    pub fn insert_vertex(&mut self, weight: V) -> usize {
        self.vertices.push(Vertex {
            weight,
            edges: vec![],
        });

        self.vertices.len() - 1
    }

    pub fn remove_edge(&mut self, index: usize) {
        self.edges.swap_remove(index);
        // in rust u cant overload the [] operator?
    }

    fn would_cycle_with_edge(&mut self, edge: Edge<E, V>) -> bool {
        self.edges.push(edge);

        let cycles = self.cycles();

        self.edges.pop();

        cycles
    }
}
impl<E: Clone, V> Edge<E, V> {
    fn new(source: usize, destination: usize, weight: E) -> Edge<E, V> {
        Edge {
            weight,
            vertices: [source, destination],
            _phantom: PhantomData,
        }
    }

    fn vertex_indices(&self) -> (usize, usize) {
        (self.vertices[0], self.vertices[1])
    }
}

impl<V: Clone> Vertex<V> {
    fn new(weight: V) -> Vertex<V> {
        Vertex {
            weight,
            edges: vec![],
        }
    }

    fn associate_edge(&mut self, index: usize) {
        match self.edges.iter().find(|&&idx| idx == index) {
            Some(_) => return,
            None => self.edges.push(index),
        }
    }

    fn disassociate_edge(&mut self, index: usize) {
        let v = self
            .edges
            .iter()
            .take_while(|&&idx| idx != index)
            .cloned()
            .collect();

        self.edges = v;
    }
}

impl<V: Clone, E: Clone> Graph<V, E, Undirected, Cyclic> {
    pub fn new_undirected_with_capacity(cap: usize) -> Self {
        Graph {
            vertices: Vec::with_capacity(cap),
            edges: Vec::with_capacity(cap),
            directed: PhantomData,
            cyclic: PhantomData,
        }
    }

    pub fn new_undirected() -> Self {
        Graph {
            vertices: vec![],
            edges: vec![],
            directed: PhantomData,
            cyclic: PhantomData,
        }
    }
}

impl<V: Clone, E: Clone> Graph<V, E, Directed, Cyclic> {
    pub fn new_directed_with_capacity(cap: usize) -> Self {
        Graph {
            vertices: Vec::with_capacity(cap),
            edges: Vec::with_capacity(cap),
            directed: PhantomData,
            cyclic: PhantomData,
        }
    }

    pub fn new_directed() -> Self {
        Graph {
            vertices: vec![],
            edges: vec![],
            directed: PhantomData,
            cyclic: PhantomData,
        }
    }
}
