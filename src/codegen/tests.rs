#[test]
fn test_build_edges() {
    use super::*;
    EdgeKindsBuilder::default()
        .digraph(
            GraphBuilder::default().name("Mir_0_4").edges(
                EdgeKindsBuilder::default()
                    .node(
                        NodeKind::Graph,
                        Some(
                            AttributesBuilder::default()
                                .font_name("Coutier, monospace")
                                .into(),
                        ),
                    )
                    .build()
                    .unwrap(),
            ),
        )
        .build()
        .unwrap();
}
