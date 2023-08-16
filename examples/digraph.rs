use graffy::codegen::{formatter::*, *};

fn main() {
    let mut formatter = Formatter::default();
    formatter.visit_statements(
        &EdgeKindsBuilder::default()
            .digraph(
                GraphBuilder::default().name("Mir_0_4").edges(
                    EdgeKindsBuilder::default()
                        .node(
                            NodeKind::Graph,
                            Some(
                                AttributesBuilder::default()
                                    .font_name("Courier, monospace")
                                    .into(),
                            ),
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
