use super::{Acyclic, Cyclic, Cyclicness, Edge, EdgeType, StableGraph, Vertex};
use std::{collections::HashSet, marker::PhantomData};

pub struct Dft<'g, V: Clone + 'g, E: Clone, C: Cyclicness> {
    vertices: Vec<Vertex<V>>,
    visited_vertices: HashSet<usize>,
    edges: Vec<Edge<E>>,
    counter: usize,
    _cyclic: PhantomData<&'g C>,
}

impl<'g, V: Clone, E: Clone, C: Cyclicness> Dft<'g, V, E, C> {
    #[inline]
    pub fn cycles(&self) -> bool {
        todo!()
    }

    #[inline]
    pub fn new<D: EdgeType>(from: &StableGraph<V, E, D, C>) -> Self {
        Self::from(from)
    }
}

impl<'g, V: Clone, E: Clone, D: EdgeType, C: Cyclicness> From<&StableGraph<V, E, D, C>>
    for Dft<'g, V, E, C>
{
    fn from(from: &StableGraph<V, E, D, C>) -> Self {
        Dft {
            vertices: from.vertices.values().cloned().collect(),
            visited_vertices: HashSet::new(),
            edges: from.edges.values().cloned().collect(),
            counter: 0,
            _cyclic: PhantomData,
        }
    }
}

impl<'g, V: Clone, E: Clone> Dft<'g, V, E, Cyclic> {
    pub fn new_cyclic<D: EdgeType, C: Cyclicness>(from: &StableGraph<V, E, D, C>) -> Self {
        Dft {
            vertices: from.vertices.values().cloned().collect(),
            visited_vertices: HashSet::new(),
            edges: from.edges.values().cloned().collect(),
            counter: 0,
            _cyclic: PhantomData,
        }
    }
}

impl<'g, V: Clone, E: Clone> Dft<'g, V, E, Acyclic> {
    pub fn new_acyclic<D: EdgeType, C: Cyclicness>(from: &StableGraph<V, E, D, C>) -> Self {
        Dft {
            vertices: from.vertices.values().cloned().collect(),
            visited_vertices: HashSet::new(),
            edges: from.edges.values().cloned().collect(),
            counter: 0,
            _cyclic: PhantomData,
        }
    }
}

impl<'g, V: Clone, E: Clone, C: Cyclicness> Iterator for Dft<'g, V, E, C> {
    type Item = &'g Vertex<V>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
