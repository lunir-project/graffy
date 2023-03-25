#![cfg(test)]

use super::*;
use crate::graph::dft::Dft;

#[test]
fn create_graph() {
    let mut graph = Graph::<usize, usize, Directed, Acyclic>::new_directed();

    let v1 = graph.insert_vertex(20);
    let v2 = graph.insert_vertex(30);
    let v3 = graph.insert_vertex(40);

    graph.insert_edge(v1, v2, 25).unwrap();
    graph.insert_edge(v2, v2, 25).unwrap();
    graph.insert_edge(v3, v1, 75).unwrap();

    let traverser = Dft::from(&graph);

    for (index, vertex) in traverser.enumerate() {
        eprintln!("{index}: {:#?}", vertex);
    }

    panic!()

    // where do we check..
}
