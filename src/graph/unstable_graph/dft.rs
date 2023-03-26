use super::unstable_graph::{Acyclic, Cyclic, Cyclicness, Edge, EdgeType, UnstableGraph, Vertex};
use std::{collections::HashSet, marker::PhantomData};

/// A depth-first traverser which yields items from an [`UnstableGraph`] on a
/// [depth-first](https://en.wikipedia.org/wiki/Depth-first_search) basis.
pub struct Dft<'g, V: Clone, E: Clone, C: Cyclicness> {
    vertices: &'g [Vertex<V>],
    visited_vertices: HashSet<usize>,
    edges: &'g [Edge<E>],
    counter: usize,
    _cyclic: PhantomData<C>,
}

impl<'g, V: Clone, E: Clone, C: Cyclicness> Dft<'g, V, E, C> {
    /// Returns if the [`UnstableGraph`] that this [`Dft`] is iterating over
    /// contains a cycle.
    #[inline]
    pub fn cycles(&self) -> bool {
        let mut visited = HashSet::new();
        let mut pending_checks = vec![];

        visited.insert(self.edges[0].vertex_indices[0]);
        pending_checks.push(self.edges[0].vertex_indices[0]);

        while !pending_checks.is_empty() {
            let vertex = pending_checks.pop().unwrap();

            for edge in &self.vertices[vertex].edge_indices {
                let adjacent = self.edges[*edge].vertex_indices[1];
                if visited.insert(adjacent) {
                    pending_checks.push(adjacent);
                } else if self.vertices[adjacent]
                    .edge_indices
                    .iter()
                    .any(|&e| e != *edge && self.edges[e].vertex_indices[1] == adjacent)
                {
                    return true;
                }
            }
        }

        false
    }

    /// Creates a new [`Dft`] from an [`UnstableGraph`]; the cyclicness of the resulting traverser
    /// is the same as the cyclicness of the underlying graph.
    #[inline]
    pub fn new<D: EdgeType>(from: &'g UnstableGraph<V, E, D, C>) -> Self {
        Self::from(from)
    }
}

impl<'g, V: Clone, E: Clone, D: EdgeType, C: Cyclicness> From<&'g UnstableGraph<V, E, D, C>>
    for Dft<'g, V, E, C>
{
    /// Creates a new [`Dft`] from an [`UnstableGraph`]; the cyclicness of the resulting traverser
    /// is the same as the cyclicness of the underlying graph.
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
    /// Creates a new cyclic [`Dft`] from an [`UnstableGraph`].
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
    /// Creates a new [`Dft`] from an [`UnstableGraph`].
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

impl<'g, V: Clone, E: Clone, C: Cyclicness> Iterator for Dft<'g, V, E, C> {
    type Item = &'g Vertex<V>;

    /// Gets the next item of the iterator.
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter >= self.vertices.len() + 1 {
            return None;
        }

        if self.counter == 0 {
            let first_vertex = self.edges[self.counter].vertex_indices[0];
            self.visited_vertices.insert(first_vertex);

            self.counter += 1;
            return Some(&self.vertices[first_vertex]);
        } else if self.counter == 1 {
            let second_vertex = self.edges[self.counter - 1].vertex_indices[1];
            self.visited_vertices.insert(second_vertex);

            self.counter += 1;
            return Some(&self.vertices[second_vertex]);
        }

        let current_edge = &self.edges[self.counter - 1];
        let destination_vertex = current_edge.vertex_indices[1];

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
