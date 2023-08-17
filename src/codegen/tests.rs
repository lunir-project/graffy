#[test]
fn test_build_edges() {
    use super::*;
    EdgeKindsBuilder::default()
        .digraph(
            GraphBuilder::default().name("Mir_0_4").edges(
                EdgeKindsBuilder::default()
                    .node(
                        NodeKind::Graph,
                        AttributesBuilder::default().font_name("Coutier, monospace"),
                    )
                    .build()
                    .unwrap(),
            ),
        )
        .build()
        .unwrap();
}
