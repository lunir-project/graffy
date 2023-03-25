mod tests;

use super::dft::*;
use std::{collections::HashSet, marker::PhantomData};
#[derive(Clone, Debug)]
pub enum Cyclic {}
#[derive(Clone, Debug)]
pub enum Acyclic {}

#[derive(Debug)]
pub struct CycleError;

pub trait Cyclicness: Sized {
    fn insert_edge<V: Clone, E: Clone, D: EdgeType>(
        graph: &mut Graph<V, E, D, Self>,
        edge: Edge<E>,
    ) -> Result<usize, CycleError>;

    fn is_cyclic() -> bool;
}

impl Cyclicness for Cyclic {
    fn insert_edge<V: Clone, E: Clone, D: EdgeType>(
        graph: &mut Graph<V, E, D, Self>,
        edge: Edge<E>,
    ) -> Result<usize, CycleError> {
        Ok(graph.insert_edge_unchecked(edge))
    }

    fn is_cyclic() -> bool {
        true
    }
}

impl Cyclicness for Acyclic {
    fn insert_edge<V: Clone, E: Clone, D: EdgeType>(
        graph: &mut Graph<V, E, D, Self>,
        edge: Edge<E>,
    ) -> Result<usize, CycleError> {
        if graph.would_cycle_with_edge(edge.clone()) {
            return Err(CycleError);
        }
        // bro no it doesnt wtf r u talking ab
        Ok(graph.insert_edge_unchecked(edge))
    }

    fn is_cyclic() -> bool {
        false
    }
}

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

pub enum Direction {
    Outgoing,
    Incoming,
}

impl Direction {
    #[inline]
    pub fn opposite(self) -> Self {
        match self {
            Self::Outgoing => Self::Incoming,
            Self::Incoming => Self::Outgoing,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Vertex<V: Clone> {
    pub weight: V,
    pub edges: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct Edge<E: Clone> {
    pub weight: E,
    pub vertices: [usize; 2],
}

impl<V: Clone> Vertex<V> {
    #[inline]
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

    #[inline]
    pub fn edge_indices(&self) -> &[usize] {
        &self.edges
    }

    #[inline]
    pub fn weight(&self) -> &V {
        &self.weight
    }
}

impl<E: Clone> Edge<E> {
    #[inline]
    fn new(source: usize, destination: usize, weight: E) -> Edge<E> {
        Edge {
            weight,
            vertices: [source, destination],
        }
    }

    fn vertex_indices(&self) -> (usize, usize) {
        (self.vertices[0], self.vertices[1])
    }
}

#[derive(Clone, Debug)]
pub struct Graph<V: Clone, E: Clone, D: EdgeType, C: Cyclicness> {
    pub vertices: Vec<Vertex<V>>,
    pub edges: Vec<Edge<E>>,
    _directed: PhantomData<D>,
    _cyclic: PhantomData<C>,
}

impl<V: Clone, E: Clone, D: EdgeType, C: Cyclicness> Graph<V, E, D, C> {
    pub fn cycles(&self) -> bool {
        Dft::from(self).cycles()
        // oh my bad ahhhhhhh i see it now, srry. ok let me redo
        // its ok
    }

    #[inline]
    pub fn insert_edge(
        &mut self,
        source: usize,
        destination: usize,
        weight: E,
    ) -> Result<usize, CycleError> {
        C::insert_edge(self, Edge::new(source, destination, weight))
    }

    fn insert_edge_unchecked(&mut self, edge: Edge<E>) -> usize {
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

    #[inline]
    pub fn insert_vertex(&mut self, weight: V) -> usize {
        self.vertices.push(Vertex {
            weight,
            edges: vec![],
        });
        self.vertices.len() - 1
    }

    #[inline]
    pub fn remove_edge(&mut self, index: usize) {
        self.edges.swap_remove(index);
    }

    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    #[inline]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    fn would_cycle_with_edge(&mut self, edge: Edge<E>) -> bool {
        self.edges.push(edge.clone());

        let cycles = self.cycles();
        self.edges.pop();

        cycles
    }
}

impl<V: Clone, E: Clone> Graph<V, E, Directed, Cyclic> {
    #[inline]
    pub fn new_directed_with_capacity(cap: usize) -> Self {
        Graph {
            vertices: Vec::with_capacity(cap),
            edges: Vec::with_capacity(cap),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }

    #[inline]
    pub fn new_directed() -> Self {
        Graph {
            vertices: vec![],
            edges: vec![],
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }
}

impl<V: Clone, E: Clone> Graph<V, E, Directed, Acyclic> {
    #[inline]
    pub fn new_directed_with_capacity(cap: usize) -> Self {
        Graph {
            vertices: Vec::with_capacity(cap),
            edges: Vec::with_capacity(cap),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }

    #[inline]
    pub fn new_directed() -> Self {
        Graph {
            vertices: vec![],
            edges: vec![],
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }
}

impl<V: Clone, E: Clone> Graph<V, E, Undirected, Cyclic> {
    #[inline]
    pub fn new_undirected_with_capacity(cap: usize) -> Self {
        Graph {
            vertices: Vec::with_capacity(cap),
            edges: Vec::with_capacity(cap),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }

    #[inline]
    pub fn new_undirected() -> Self {
        Graph {
            vertices: vec![],
            edges: vec![],
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }
}
