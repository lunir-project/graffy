use std::rc::Rc;

use super::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FormatterSettings {
    /// Pretty print with indentation and new lines
    #[default]
    Pretty,

    /// Minimal new lines and whitespace
    Minified,
}

#[derive(Clone, PartialEq, Default)]
pub struct Formatter {
    buffer: String,
    depth: Rc<()>,
    settings: FormatterSettings,

    last_graph: Option<GraphKind>,
    current_graph: Option<GraphKind>,
}

impl Formatter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_settings(settings: FormatterSettings) -> Self {
        Self {
            settings,
            ..Self::default()
        }
    }

    pub fn source(self) -> String {
        self.buffer
    }

    fn indent(&self) -> String {
        "    ".repeat(Rc::strong_count(&self.depth) - 1)
    }

    fn pretty_or<'a>(&self, pretty: &'a str, other: &'a str) -> &'a str {
        match self.settings {
            FormatterSettings::Minified => other,
            FormatterSettings::Pretty => pretty,
        }
    }

    pub fn visit_statements(&mut self, stmts: &Vec<EdgeKind>) {
        for stat in stmts {
            self.buffer.push_str(&self.indent());
            self.visit_edge_kind(&stat);
            self.buffer.push_str(self.pretty_or("", "\n"));
        }
    }

    fn visit_block(&mut self, stmts: &Vec<EdgeKind>) {
        self.buffer.push_str(self.pretty_or(" {\n", "{"));
        {
            let _scope = self.depth.clone();
            self.visit_statements(stmts);
        }

        self.buffer
            .push_str(self.pretty_or(&format!("\n{}}}", self.indent()), "}"));
    }

    fn visit_node_kind(&mut self, node: &NodeKind) {
        self.buffer.push_str(match node {
            NodeKind::Edge => "edge",
            NodeKind::Graph => "graph",
            NodeKind::Node => "node",
            NodeKind::Custom(s) => &s,
        });
    }

    fn visit_graph_kind(&mut self, node: &GraphKind) {
        self.last_graph = self.current_graph.clone();
        self.current_graph = Some(node.clone());

        match node {
            GraphKind::Directed(graph) => self.visit_graph(graph, true),
            GraphKind::Undirected(graph) => self.visit_graph(graph, false),
        }

        self.current_graph = self.last_graph.clone();
    }

    fn visit_graph(&mut self, node: &Graph, directed: bool) {
        if node.is_strict {
            self.buffer.push_str("strict ");
        }

        self.buffer
            .push_str(if directed { "digraph" } else { "graph" });

        if let Some(name) = &node.name {
            self.buffer.push_str(&format!(" {name}"))
        }

        self.visit_attributes(&node.attributes);
        self.visit_block(&node.edges);
    }

    fn visit_edge_kind(&mut self, edge: &EdgeKind) {
        match edge {
            EdgeKind::Node(kind, attrs) => self.visit_node(kind, attrs),
            EdgeKind::Edge(expr) => self.visit_edge(expr),
            EdgeKind::Comment(text) => self.visit_comment(&text),
            EdgeKind::SubGraph(body) => {
                self.buffer.push_str("subgraph");
                self.visit_block(&body);
            }
            EdgeKind::Graph(graph) => self.visit_graph_kind(graph),
        }

        match edge {
            EdgeKind::Node(..) | EdgeKind::Edge(..) => self.buffer.push(';'),
            _ => {}
        }
    }

    fn visit_edge(&mut self, edge: &Edge) {
        self.visit_edge_kind(&edge.left);

        let arrow = if let Some(graph) = &self.current_graph {
            if matches!(graph, GraphKind::Directed(..)) {
                "->"
            } else {
                "--"
            }
        } else {
            "->"
        };

        self.buffer
            .push_str(self.pretty_or(&format!(" {arrow} ",), arrow));
        self.visit_edge_kind(&edge.right);

        if let Some(attributes) = edge.attributes.as_ref() {
            self.visit_attributes(attributes);
        }
    }

    fn visit_node(&mut self, node_kind: &NodeKind, attributes: &Option<Vec<Attribute>>) {
        self.visit_node_kind(node_kind);

        if let Some(attributes) = attributes {
            self.visit_attributes(attributes);
        }
    }

    fn visit_comment(&mut self, text: &str) {
        // a pretty printer will put the comment node in a new line anyway.
        self.buffer
            .push_str(&format!("// {text}{}", self.pretty_or("", "\n")));
    }

    fn visit_attributes(&mut self, attributes: &Vec<Attribute>) {
        if !attributes.is_empty() {
            self.buffer.push_str(self.pretty_or(" [", "["));

            let last = attributes.len() - 1;
            for (i, attr) in attributes.iter().enumerate() {
                self.visit_attribute(attr);

                if i < last {
                    self.buffer.push_str(self.pretty_or(", ", ","));
                }
            }

            self.buffer.push(']');
        }
    }

    fn visit_attribute(&mut self, attribute: &Attribute) {
        self.buffer.push_str(&match attribute {
            Attribute::FontName(name) => format!(r#"fontname="{name}""#),
        });
    }
}
