use graffy::codegen::{formatter::*, *};

fn main() {
    let mut formatter = Formatter::default();
    formatter.visit_statements(
        &EdgeKindsBuilder::default()
            .graph(
                GraphBuilder::default().name("AttributeGraph").edges(
                    EdgeKindsBuilder::default()
                        .node(
                            NodeKind::Node,
                            Some(
                                AttributesBuilder::default()
                                    .shape(ShapeKind::Ellipse)
                                    .color(ColorKind::Blue)
                                    .into(),
                            ),
                        )
                        .node(
                            NodeKind::Custom("A".into()),
                            Some(AttributesBuilder::default().label("Node A").into()),
                        )
                        .node(
                            NodeKind::Custom("B".into()),
                            Some(AttributesBuilder::default().label("Node B").into()),
                        )
                        .edge(
                            EdgeBuilder::default()
                                .left(NodeKind::Custom("A".into()))
                                .right(NodeKind::Custom("B".into()))
                                .attributes(AttributesBuilder::default().label("Edge from A to B")),
                        )
                        .build()
                        .unwrap(),
                ),
            )
            .build()
            .unwrap(),
    );

    println!("{}", formatter.source());
}
