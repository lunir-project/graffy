use super::unstable_graph::{Acyclic, Cyclic, Cyclicness, Edge, EdgeType, UnstableGraph, Vertex};
use std::{collections::HashSet, marker::PhantomData};

pub struct Dft<'g, V: Clone, E: Clone, C: Cyclicness> {
    vertices: &'g [Vertex<V>],
    visited_vertices: HashSet<usize>,
    edges: &'g [Edge<E>],
    counter: usize,
    _cyclic: PhantomData<C>,
}

impl<'g, V: Clone, E: Clone, C: Cyclicness> Dft<'g, V, E, C> {
    #[inline]
    pub fn cycles(&self) -> bool {
        let mut visited = HashSet::new();
        let mut pending_checks = vec![];

        visited.insert(self.edges[0].vertices[0]);
        pending_checks.push(self.edges[0].vertices[0]);

        while !pending_checks.is_empty() {
            let vertex = pending_checks.pop().unwrap();

            for edge in &self.vertices[vertex].edges {
                let adjacent = self.edges[*edge].vertices[1];
                if visited.insert(adjacent) {
                    pending_checks.push(adjacent);
                } else if self.vertices[adjacent]
                    .edges
                    .iter()
                    .any(|&e| e != *edge && self.edges[e].vertices[1] == adjacent)
                {
                    return true;
                }
            }
        }

        false
    }

    #[inline]
    pub fn new<D: EdgeType>(from: &'g UnstableGraph<V, E, D, C>) -> Self {
        Self::from(from)
    }
}

impl<'g, V: Clone, E: Clone, D: EdgeType, C: Cyclicness> From<&'g UnstableGraph<V, E, D, C>>
    for Dft<'g, V, E, C>
{
    fn from(from: &'g UnstableGraph<V, E, D, C>) -> Self {
        Dft {
            vertices: &from.vertices.as_slice(),
            visited_vertices: HashSet::new(),
            edges: &from.edges,
            counter: 0,
            _cyclic: PhantomData,
        }
    }
}

impl<'g, V: Clone, E: Clone> Dft<'g, V, E, Cyclic> {
    pub fn new_cyclic<D: EdgeType, C: Cyclicness>(from: &'g UnstableGraph<V, E, D, C>) -> Self {
        Dft {
            vertices: &from.vertices.as_slice(),
            visited_vertices: HashSet::new(),
            edges: &from.edges,
            counter: 0,
            _cyclic: PhantomData,
        }
    }
}

impl<'g, V: Clone, E: Clone> Dft<'g, V, E, Acyclic> {
    pub fn new_acyclic<D: EdgeType, C: Cyclicness>(from: &'g UnstableGraph<V, E, D, C>) -> Self {
        Dft {
            vertices: &from.vertices.as_slice(),
            visited_vertices: HashSet::new(),
            edges: &from.edges,
            counter: 0,
            _cyclic: PhantomData,
        }
    }
}

// TODO: StableGraph From impl

impl<'g, V: Clone, E: Clone, C: Cyclicness> Iterator for Dft<'g, V, E, C> {
    type Item = &'g Vertex<V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter >= self.vertices.len() + 1 {
            return None;
        }

        if self.counter == 0 {
            let first_vertex = self.edges[self.counter].vertices[0];
            self.visited_vertices.insert(first_vertex);

            self.counter += 1;
            return Some(&self.vertices[first_vertex]);
        } else if self.counter == 1 {
            let second_vertex = self.edges[self.counter - 1].vertices[1];
            self.visited_vertices.insert(second_vertex);

            self.counter += 1;
            return Some(&self.vertices[second_vertex]);
        }

        let current_edge = &self.edges[self.counter - 1];
        let destination_vertex = current_edge.vertices[1];

        self.counter += 1;
        if !C::is_cyclic() {
            if !self.visited_vertices.insert(destination_vertex) {
                return None;
            }
        }

        self.visited_vertices.insert(destination_vertex);
        Some(&self.vertices[destination_vertex])
    }
}

// TODO: implement iterator for Dft<stable_graph::StableGraph>*/
