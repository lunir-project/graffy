#![cfg(test)]

use super::*;
use crate::graph::dft::Dft;

#[test]
fn create_graph() {
    let mut graph = Graph::<usize, usize, Directed, Cyclic>::new_directed();

    let vertex2 = graph.insert_vertex(20);
    let vertex3 = graph.insert_vertex(30);
    let vertex1 = graph.insert_vertex(10);
    let vertex4 = graph.insert_vertex(40);

    graph.insert_edge(vertex2, vertex3, 25).unwrap();
    graph.insert_edge(vertex3, vertex1, 50).unwrap();
    graph.insert_edge(vertex1, vertex2, 75).unwrap();

    let traverser = Dft::from(&graph);

    for (index, vertex) in traverser.enumerate() {
        println!("{index}: {:#?}", vertex);
    }

    // where do we check..
}
