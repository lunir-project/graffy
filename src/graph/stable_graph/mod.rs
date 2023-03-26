mod dft;
mod tests;

use std::marker::PhantomData;

use slotmap::{DefaultKey, DenseSlotMap};

use super::CycleError;
#[derive(Clone, Debug)]
pub enum Cyclic {}
#[derive(Clone, Debug)]
pub enum Acyclic {}

#[derive(Debug)]
struct InvalidKey;

pub trait Cyclicness: Sized {
    fn insert_edge<V: Clone, E: Clone, D: EdgeType>(
        graph: &mut StableGraph<V, E, D, Self>,
        edge: Edge<E>,
    ) -> Result<DefaultKey, CycleError>;

    fn is_cyclic() -> bool;
}

impl Cyclicness for Cyclic {
    fn insert_edge<V: Clone, E: Clone, D: EdgeType>(
        graph: &mut StableGraph<V, E, D, Self>,
        edge: Edge<E>,
    ) -> Result<DefaultKey, CycleError> {
        Ok(graph.insert_edge_unchecked(edge))
    }

    fn is_cyclic() -> bool {
        true
    }
}

impl Cyclicness for Acyclic {
    fn insert_edge<V: Clone, E: Clone, D: EdgeType>(
        graph: &mut StableGraph<V, E, D, Self>,
        edge: Edge<E>,
    ) -> Result<DefaultKey, CycleError> {
        if graph.would_cycle_with_edge(edge.clone()) {
            return Err(CycleError);
        }
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
    pub edges: Vec<DefaultKey>,
}

#[derive(Clone, Debug)]
pub struct Edge<E: Clone> {
    pub weight: E,
    pub vertices: [DefaultKey; 2],
}

impl<V: Clone> Vertex<V> {
    #[inline]
    fn new(weight: V) -> Vertex<V> {
        Vertex {
            weight,
            edges: vec![],
        }
    }

    fn associate_edge(&mut self, key: DefaultKey) {
        match self.edges.iter().find(|&&idx| idx == key) {
            Some(_) => return,
            None => self.edges.push(key),
        }
    }

    fn dissociate_edge(&mut self, key: DefaultKey) {
        let v = self
            .edges
            .iter()
            .filter(|&&idx| idx != key)
            .cloned()
            .collect();

        self.edges = v;
    }

    #[inline]
    pub fn edge_indices(&self) -> &[DefaultKey] {
        &self.edges
    }

    #[inline]
    pub fn weight(&self) -> &V {
        &self.weight
    }
}

impl<E: Clone> Edge<E> {
    #[inline]
    fn new(source: DefaultKey, destination: DefaultKey, weight: E) -> Edge<E> {
        Edge {
            weight,
            vertices: [source, destination],
        }
    }

    fn vertex_indices(&self) -> (DefaultKey, DefaultKey) {
        (self.vertices[0], self.vertices[1])
    }
}

#[derive(Clone, Debug)]
pub struct StableGraph<V: Clone, E: Clone, D: EdgeType, C: Cyclicness> {
    pub vertices: DenseSlotMap<DefaultKey, Vertex<V>>,
    pub edges: DenseSlotMap<DefaultKey, Edge<E>>,
    _directed: PhantomData<D>,
    _cyclic: PhantomData<C>,
}

impl<V: Clone, E: Clone, D: EdgeType, C: Cyclicness> StableGraph<V, E, D, C> {
    pub fn cycles(&self) -> bool {
        todo!()
        // Dft::from(self).cycles()
    }

    #[inline]
    pub fn insert_edge(
        &mut self,
        source: DefaultKey,
        destination: DefaultKey,
        weight: E,
    ) -> Result<DefaultKey, CycleError> {
        C::insert_edge(self, Edge::new(source, destination, weight))
    }

    fn insert_edge_unchecked(&mut self, edge: Edge<E>) -> DefaultKey {
        let (from, to) = edge.vertex_indices();
        let key = self.edges.insert(edge);

        match self.vertices.get_mut(from) {
            Some(source_vertex) => {
                source_vertex.associate_edge(key);
            }

            _ => panic!("Invalid key in edge upon insertion into graph."),
        };

        match self.vertices.get_mut(to) {
            Some(target_vertex) => {
                target_vertex.associate_edge(key);
            }

            _ => panic!("Invalid key in edge upon insertion into graph."),
        };

        key
    }

    #[inline]
    pub fn insert_vertex(&mut self, weight: V) -> DefaultKey {
        self.vertices.insert(Vertex {
            weight,
            edges: vec![],
        })
    }

    #[inline]
    pub fn remove_edge(&mut self, key: DefaultKey) -> Result<(), InvalidKey> {
        self.edges.remove(key).ok_or(InvalidKey).map(|_| ())
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
        let k = self.edges.insert(edge.clone());

        let cycles = self.cycles();
        self.edges.remove(k).unwrap();

        cycles
    }
}

impl<V: Clone, E: Clone> StableGraph<V, E, Directed, Cyclic> {
    #[inline]
    pub fn new_directed_with_capacity(cap: usize) -> Self {
        StableGraph {
            vertices: DenseSlotMap::with_capacity(cap),
            edges: DenseSlotMap::with_capacity(cap),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }

    #[inline]
    pub fn new_directed() -> Self {
        StableGraph {
            vertices: DenseSlotMap::new(),
            edges: DenseSlotMap::new(),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }
}

impl<V: Clone, E: Clone> StableGraph<V, E, Directed, Acyclic> {
    #[inline]
    pub fn new_directed_with_capacity(cap: usize) -> Self {
        StableGraph {
            vertices: DenseSlotMap::with_capacity(cap),
            edges: DenseSlotMap::with_capacity(cap),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }

    #[inline]
    pub fn new_directed() -> Self {
        StableGraph {
            vertices: DenseSlotMap::new(),
            edges: DenseSlotMap::new(),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }
}

impl<V: Clone, E: Clone> StableGraph<V, E, Undirected, Cyclic> {
    #[inline]
    pub fn new_undirected_with_capacity(cap: usize) -> Self {
        StableGraph {
            vertices: DenseSlotMap::with_capacity(cap),
            edges: DenseSlotMap::with_capacity(cap),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }

    #[inline]
    pub fn new_undirected() -> Self {
        StableGraph {
            vertices: DenseSlotMap::new(),
            edges: DenseSlotMap::new(),
            _directed: PhantomData,
            _cyclic: PhantomData,
        }
    }
}
