use super::graph::{Acyclic, Cyclic, Cyclicness, Edge, EdgeType, Graph, Vertex};
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

        // here we're pushing the first source vertex of the entire graph to both
        // visited and pending. :thumbsup:
        visited.insert(self.edges[0].vertices[0]);
        pending_checks.push(self.edges[0].vertices[0]);

        // here we check if pending is empty, obviously it isn't so we continue :thumbsup:
        while !pending_checks.is_empty() {
            let vertex = pending_checks.pop().unwrap();

            for edge in &self.vertices[vertex].edges {
                let adjacent = self.edges[*edge].vertices[1]; // wait a sec
                if visited.insert(adjacent) {
                    // whats wrong, is it not working?
                    pending_checks.push(adjacent);
                } else if self.vertices[adjacent]
                    .edges
                    .iter()
                    .any(|&e| e != *edge && self.edges[e].vertices[1] == adjacent)
                // ok im either a genius or dumb. it's working, no more cycle error
                // it is cycling tho?
                // no, same code that u had written
                //
                // brother below we go from 4 -> 1
                // once we hook up 3 then the cycle is completed
                // hang on i cant compile
                {
                    return true;
                }
            }
        }

        false
    }

    #[inline]
    pub fn new<D: EdgeType>(from: &'g Graph<V, E, D, C>) -> Self {
        Self::from(from)
    }
}

impl<'g, V: Clone, E: Clone, D: EdgeType, C: Cyclicness> From<&'g Graph<V, E, D, C>>
    for Dft<'g, V, E, C>
{
    fn from(from: &'g Graph<V, E, D, C>) -> Self {
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
    fn new_cyclic<D: EdgeType, C: Cyclicness>(from: &'g Graph<V, E, D, C>) -> Self {
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
    fn new_acyclic<D: EdgeType, C: Cyclicness>(from: &'g Graph<V, E, D, C>) -> Self {
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
        //if C::is_cyclic() {} // use this foe what
        // knowing if you should keep looping in a cycle or not

        if self.counter >= self.vertices.len() {
            return None;
        }

        println!("Counter: {}", self.counter);
        self.counter += 1;
        // if this is the first iteration, return the first vertex
        if self.counter - 1 == 0 {
            return Some(&self.vertices[self.edges[self.counter - 1].vertices[0]]);
        } else if self.counter - 1 == 1 {
            // since the second edge will have the third vertex as the destination
            // we need to check if we are on the second iteration and return the second vertex.
            return Some(&self.vertices[self.edges[self.counter - 2].vertices[1]]);
        }
        // here we just need to go destination to destination to destination one after the other
        // i managed to do it, but sadly if it's cyclic, it will output the destination even though
        // it has already been outputted previously (side effect of having a cyclic graph)
        // so we need to fix that behavior and check for any other possible edge cases.
        // good?
        let current_edge = &self.edges[self.counter - 2];
        let destination_vertex = current_edge.vertices[1];

        if !C::is_cyclic() {
            if !self.visited_vertices.insert(destination_vertex) {
                return None;
            }
        }

        println!(
            "destination: {destination_vertex}, counter: {}",
            self.counter - 2
        );
        Some(&self.vertices[destination_vertex])
    }
}

// TODO: implement iterator for Dft<stable_graph::StableGraph>*/
