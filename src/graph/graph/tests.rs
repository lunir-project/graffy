#![cfg(test)]

use crate::graph;

use super::*;

#[test]
fn create_graph() {
    let mut graph = Graph::new_directed();

    let index_1 = graph.insert_vertex(50_usize);
    let index_2 = graph.insert_vertex(59_usize);

    graph.insert_edge_unchecked(Edge {
        weight: (),
        vertices: dbg!([index_1, index_2]),
        _phantom: PhantomData,
    });

    println!("{:#?}", graph);
    // where do we check..
}
