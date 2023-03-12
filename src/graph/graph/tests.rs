#![cfg(test)]

use super::*;

#[test]
fn create_graph() {
    let mut graph = Graph::<i32, (), Directed, Cyclic>::new_directed();

    graph.add_vertex(50);
    graph.add_vertex(34);
    println!("{:#?}", graph);
}
