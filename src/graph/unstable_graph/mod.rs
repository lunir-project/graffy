mod dft;
mod tests;

use super::*;
use dft::*;
use std::marker::PhantomData;

/// Uninhabited type that indicates that an [`UnstableGraph`] is cyclic.
#[derive(Clone, Debug)]
pub enum Cyclic {}

/// Uninhabited type that indicates that [`UnstableGraph`] is acyclic.
#[derive(Clone, Debug)]
pub enum Acyclic {}

/// A trait for [`UnstableGraph`]'s cyclicness types.
pub trait Cyclicness: Sized {
    /// The edge insertion behaviour for cyclic or acyclic [`UnstableGraph]`s.
    ///
    /// # Fallible:
    /// Returns an `Err` if the [`UnstableGraph`] being inserted to is acyclic and the insertion
    /// of this [`Edge`] would introduce a cycle.
    fn insert_edge<V: Clone, E: Clone, D: EdgeType>(
        graph: &mut UnstableGraph<V, E, D, Self>,
        edge: Edge<E>,
    ) -> Result<usize, CycleError>;

    /// Returns `true` if the type is cyclic.
    fn is_cyclic() -> bool;
}

impl Cyclicness for Cyclic {
    /// The edge insertion behaviour for cyclic [`UnstableGraph`]s.
    fn insert_edge<V: Clone, E: Clone, D: EdgeType>(
        graph: &mut UnstableGraph<V, E, D, Self>,
        edge: Edge<E>,
    ) -> Result<usize, CycleError> {
        Ok(graph.insert_edge_unchecked(edge))
    }

    /// Returns `true` if the type is cyclic. Always returns `true`.
    fn is_cyclic() -> bool {
        true
    }
}

impl Cyclicness for Acyclic {
    /// The edge insertion behaviour for cyclic [`UnstableGraph`]s.
    fn insert_edge<V: Clone, E: Clone, D: EdgeType>(
        graph: &mut UnstableGraph<V, E, D, Self>,
        edge: Edge<E>,
    ) -> Result<usize, CycleError> {
        if graph.would_cycle_with_edge(edge.clone()) {
            return Err(CycleError);
        }
        Ok(graph.insert_edge_unchecked(edge))
    }

    /// Returns `true` if the type is cyclic. Always returns `false`.
    fn is_cyclic() -> bool {
        false
    }
}

/// The vertex type for [`UnstableGraph`].
#[derive(Clone, Debug)]
pub struct Vertex<V: Clone> {
    pub weight: V,
    edge_indices: Vec<usize>,
}

/// The edge type for [`UnstableGraph`].
#[derive(Clone, Debug)]
pub struct Edge<E: Clone> {
    pub weight: E,
    vertex_indices: [usize; 2],
}

impl<V: Clone> Vertex<V> {
    fn associate_edge(&mut self, index: usize) {
        match self.edge_indices.iter().find(|&&idx| idx == index) {
            Some(_) => return,
            None => self.edge_indices.push(index),
        }
    }

    fn dissociate_edge(&mut self, index: usize) {
        self.edge_indices.retain(|&idx| idx != index);
    }

    /// Gets the indices of the [`Edge`]s that are connected to this [`Vertex`].
    #[inline]
    pub fn edge_indices(&self) -> &[usize] {
        &self.edge_indices
    }

    #[inline]
    fn new(weight: V) -> Vertex<V> {
        Vertex {
            weight,
            edge_indices: vec![],
        }
    }
}

impl<E: Clone> Edge<E> {
    #[inline]
    fn new(source: usize, destination: usize, weight: E) -> Edge<E> {
        Edge {
            weight,
            vertex_indices: [source, destination],
        }
    }

    /// Gets the indices of the [`Vertices`] that are connected to this [`Edge`].
    pub fn vertex_indices(&self) -> (usize, usize) {
        (self.vertex_indices[0], self.vertex_indices[1])
    }
}

#[derive(Clone, Debug)]
pub struct UnstableGraph<V: Clone, E: Clone, D: EdgeType, C: Cyclicness> {
    _cyclic: PhantomData<C>,
    _directed: PhantomData<D>,
    pub edges: Vec<Edge<E>>,
    pub vertices: Vec<Vertex<V>>,
}

impl<V: Clone, E: Clone, D: EdgeType, C: Cyclicness> UnstableGraph<V, E, D, C> {
    /// Returns `true` if this [`UnstableGraph`] contains a cycle.
    pub fn cycles(&self) -> bool {
        Dft::from(self).cycles()
    }

    /// Returns the number of edges in this [`UnstableGraph`].
    #[inline]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Inserts an [`Edge`] into this [`UnstableGraph`].
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

    /// Inserts a [`Vertex`] into this [`UnstableGraph`].
    #[inline]
    pub fn insert_vertex(&mut self, weight: V) -> usize {
        self.vertices.push(Vertex::new(weight));
        self.vertices.len() - 1
    }

    /// Removes the [`Edge`] at `index` from this [`UnstableGraph`] while associating edges with
    /// and dissociating edges from [`Vertex`]es as necessary. This method incurs greater overhead
    /// relative to its counterpart [`remove_edge_simple`][`Self::remove_edge_simple()`], but is needed
    /// to prevent the loss of an [`UnstableGraph`]'s meaning as a result of modifying its shape.
    #[inline]
    pub fn remove_edge(&mut self, index: usize) {
        let top_index = self.edges.len() - 1;
        let top = &self.edges[top_index];

        for idx in top.vertex_indices {
            let vertex = &mut self.vertices[idx];
            vertex.dissociate_edge(top_index);
            vertex.associate_edge(index)
        }

        self.remove_edge_simple(index);
    }

    /// Removes the [`Edge`] at `index` from this [`UnstableGraph`]. Note that this is may
    /// lead to unintended consequences as the [`Vertex`]es that are associated with the [`Edge`]
    /// at `index` may become associated with  another [`Edge`] at the same index. To avoid
    /// this, use [`remove_edge`][`Self::remove_edge()`].
    #[inline]
    pub fn remove_edge_simple(&mut self, index: usize) {
        self.edges.swap_remove(index);
    }

    /// Removes the [`Vertex`] at `index` from this [`UnstableGraph`] while reconfiguring and
    /// removing [`Edge`]s as necessary. Any edges that were connected to the [`Vertex`] at `index`
    /// will be removed.  This method incurs considerable overhead relative to its counterpart
    /// [`remove_vertex_simple`][`Self::remove_vertex_simple()`], but is needed to prevent the
    /// loss of an [`UnstableGraph`]'s meaning as a result of modifying its shape.
    #[inline]
    pub fn remove_vertex(&mut self, index: usize) {
        let top_index = self.vertices.len() - 1;
        let top = self.vertices[top_index].clone();
        let target = self.vertices[index].clone();

        for idx in target.edge_indices {
            self.remove_edge(idx);
        }

        for idx in top.edge_indices {
            self.edges[idx].vertex_indices = self.edges[idx]
                .vertex_indices
                .iter()
                .map(|&ix| if ix == top_index { index } else { ix })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        }

        self.remove_edge_simple(index);
    }

    /// Removes the [`Vertex`] at `index` from this [`UnstableGraph`]. Note that this is may lead to
    /// unintended consequences as the [`Edge`]s that point to the [`Vertex`] at `index`
    /// may point to another [`Vertex`] at the same index. To avoid this, use [`remove_vertex`][`Self::remove_vertex()`].
    #[inline]
    pub fn remove_vertex_simple(&mut self, index: usize) {
        self.vertices.swap_remove(index);
    }

    /// Returns the number of vertices in this [`UnstableGraph`].
    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    fn would_cycle_with_edge(&mut self, edge: Edge<E>) -> bool {
        self.edges.push(edge.clone());

        let cycles = self.cycles();
        self.edges.pop();

        cycles
    }
}

impl<V: Clone, E: Clone> UnstableGraph<V, E, Directed, Cyclic> {
    /// Creates a new directed cyclic [`UnstableGraph`].
    #[inline]
    pub fn directed() -> Self {
        UnstableGraph {
            vertices: vec![],
            edges: vec![],
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }

    /// Creates a new directed cyclic [`UnstableGraph`] with a preallocated
    /// capacity for `cap` vertices and `cap` edges.
    #[inline]
    pub fn directed_with_capacity(cap: usize) -> Self {
        UnstableGraph {
            vertices: Vec::with_capacity(cap),
            edges: Vec::with_capacity(cap),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }

    /// Creates a new directed cyclic [`UnstableGraph`] with a preallocated
    /// capacity for `cap` vertices and approximately `cap` * `ratio` edges.
    /// This method can be used to further optimise to reduce allocations when the
    /// approximate ratio of vertices to edges is known in addition to the approximate
    /// final number of vertices.
    pub fn directed_with_capacity_and_factor(cap: usize, ratio: f32) -> Self {
        let edge_cap = (cap as f32 * ratio) as usize;

        UnstableGraph {
            vertices: Vec::with_capacity(cap),
            edges: Vec::with_capacity(edge_cap),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }
}

impl<V: Clone, E: Clone> UnstableGraph<V, E, Directed, Acyclic> {
    /// Creates a new directed acyclic [`UnstableGraph`].
    #[inline]
    pub fn directed() -> Self {
        UnstableGraph {
            vertices: vec![],
            edges: vec![],
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }

    /// Creates a new directed acyclic [`UnstableGraph`] with a preallocated
    /// capacity for `cap` vertices and `cap` edges.
    #[inline]
    pub fn directed_with_capacity(cap: usize) -> Self {
        UnstableGraph {
            vertices: Vec::with_capacity(cap),
            edges: Vec::with_capacity(cap),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }

    /// Creates a new directed acyclic [`UnstableGraph`] with a preallocated
    /// capacity for `cap` vertices and approximately `cap` * `ratio` edges.
    /// This method can be used to further optimise to reduce allocations when the
    /// approximate ratio of vertices to edges is known in addition to the approximate
    /// final number of vertices.
    pub fn directed_with_capacity_and_factor(cap: usize, ratio: f32) -> Self {
        let edge_cap = (cap as f32 * ratio) as usize;

        UnstableGraph {
            vertices: Vec::with_capacity(cap),
            edges: Vec::with_capacity(edge_cap),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }
}

impl<V: Clone, E: Clone> UnstableGraph<V, E, Undirected, Cyclic> {
    /// Creates a new undirected cyclic [`UnstableGraph`].
    #[inline]
    pub fn undirected() -> Self {
        UnstableGraph {
            vertices: vec![],
            edges: vec![],
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }

    /// Creates a new undirected cyclic [`UnstableGraph`] with a preallocated
    /// capacity for `cap` vertices and `cap` edges.
    #[inline]
    pub fn undirected_with_capacity(cap: usize) -> Self {
        UnstableGraph {
            vertices: Vec::with_capacity(cap),
            edges: Vec::with_capacity(cap),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }

    /// Creates a new undirected cyclic [`UnstableGraph`] with a preallocated
    /// capacity for `cap` vertices and approximately `cap * ratio` edges.
    /// This method can be used to further optimise to reduce allocations when the
    /// approximate ratio of vertices to edges is known in addition to the approximate
    /// final number of vertices.
    pub fn undirected_with_capacity_and_factor(cap: usize, ratio: f32) -> Self {
        let edge_cap = (cap as f32 * ratio) as usize;

        UnstableGraph {
            vertices: Vec::with_capacity(cap),
            edges: Vec::with_capacity(edge_cap),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }
}
