use graffy::codegen::{formatter::*, *};

fn main() {
    let mut formatter = Formatter::with_settings(FormatterSettings::Pretty);
    formatter.visit_statements(
        EdgeKindsBuilder::default().graph(
            GraphKind::builder().name("H").edges(
                EdgeKindsBuilder::default()
                    .subgraph(
                        SubGraph::builder().name("G").body(
                            EdgeKindsBuilder::default()
                                .node("C", AttributesBuilder::default().label("duude"))
                                .node("D", AttributesBuilder::default().label("howww"))
                                .edge(EdgeBuilder::default().left("C").right("D")),
                        ),
                    )
                    .node("A", AttributesBuilder::default().label("IDK"))
                    .node("B", AttributesBuilder::default().label("ANYMORE"))
                    .edge(EdgeBuilder::default().left("A").right("B")),
            ),
        ),
    );

    println!("{}", formatter.source());
}
