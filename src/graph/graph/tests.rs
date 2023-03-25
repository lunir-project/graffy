#![cfg(test)]

use super::*;
use crate::graph::dft::Dft;

#[test]
fn create_graph() { 
    let mut graph = Graph::<usize, usize, Directed, Cyclic>::new_directed();

    
    let v1 = graph.insert_vertex(10);
    let v2 = graph.insert_vertex(20);
    let v3 = graph.insert_vertex(30);
    let v4 = graph.insert_vertex(40);

    graph.insert_edge(v1, v2, 25).unwrap();
    graph.insert_edge(v2, v3, 50).unwrap();
    graph.insert_edge(v3, v4, 75).unwrap();
    graph.insert_edge(v4, v3, 100).unwrap();


    let traverser = Dft::from(&graph);

    for (index, vertex) in traverser.enumerate() {
        eprintln!("{index}: {:#?}", vertex);
    }
}
