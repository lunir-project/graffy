mod tests;

/// Builder trait for all builders of `T`
pub trait Builder<T>: Into<T> + Default {
    /// consume `self` and try to build `T`, may fail
    fn build(self) -> Option<T>;
}

macro_rules! impl_into_underlying {
    ($kind: ty, $builder: ty) => {
        impl Into<$kind> for $builder {
            fn into(self) -> $kind {
                self.build().expect(&format!(
                    "[{}]: could not build type of `{}`",
                    std::any::type_name::<$builder>(),
                    std::any::type_name::<$kind>()
                ))
            }
        }
    };
}

/// Undirected edge digraph
#[derive(Debug, Clone, PartialEq)]
pub struct Graph {
    name: Option<String>,
    edges: Vec<EdgeKind>,
    is_strict: bool,
    attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GraphKind {
    Undirected(Graph),
    Directed(Graph),
}

impl Into<Graph> for GraphKind {
    fn into(self) -> Graph {
        match self {
            Self::Directed(graph) | Self::Undirected(graph) => graph,
        }
    }
}

/// Builder for a directed edge graph
#[derive(Default)]
pub struct GraphBuilder {
    name: Option<String>,
    edges: Vec<EdgeKind>,
    is_strict: Option<bool>,
    attributes: Vec<Attribute>,
    is_directed: bool,
}
impl_into_underlying!(GraphKind, GraphBuilder);

impl Builder<GraphKind> for GraphBuilder {
    fn build(self) -> Option<GraphKind> {
        let graph = Graph {
            name: self.name,
            edges: self.edges,
            is_strict: self.is_strict.unwrap_or(false),
            attributes: self.attributes,
        };

        Some(if self.is_directed {
            GraphKind::Directed(graph)
        } else {
            GraphKind::Undirected(graph)
        })
    }
}

impl GraphBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn edges(mut self, edges: impl AsRef<[EdgeKind]>) -> Self {
        self.edges = edges.as_ref().to_vec();
        self
    }

    pub fn strict(mut self) -> Self {
        self.is_strict = Some(true);
        self
    }

    pub fn directed(mut self) -> Self {
        self.is_directed = true;
        self
    }

    pub fn undirected(mut self) -> Self {
        self.is_directed = false;
        self
    }
}

/// Attributes for edges
#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    /// fontname
    FontName(Option<String>),
}

/// Builder for multiple attributes
#[derive(Default)]
pub struct AttributesBuilder(Vec<Attribute>);
impl_into_underlying!(Vec<Attribute>, AttributesBuilder);

impl Builder<Vec<Attribute>> for AttributesBuilder {
    fn build(self) -> Option<Vec<Attribute>> {
        Some(self.0)
    }
}

impl AttributesBuilder {
    pub fn font_name(mut self, f: impl Into<String>) -> Self {
        self.0.push(Attribute::FontName(Some(f.into())));
        self
    }
}

/// Kinds of `Edge`s and their data
#[derive(Debug, Clone, PartialEq)]
pub enum EdgeKind {
    /// `Node(name, attributes)` -> `name [attributes]`
    Node(NodeKind, Option<Vec<Attribute>>),

    /// `Edge(edge)` -> `edge.left -> edge.right [edge.attributes]`
    Edge(Box<Edge>),

    /// `SubGraph(body)` -> `subgraph { body }`
    SubGraph(Vec<EdgeKind>),

    /// `Comment(text)` -> `// text`
    Comment(String),

    Graph(GraphKind),
}

/// Builder for edge kinds
#[derive(Default)]
pub struct EdgeKindsBuilder(Vec<EdgeKind>);
impl_into_underlying!(Vec<EdgeKind>, EdgeKindsBuilder);

impl Builder<Vec<EdgeKind>> for EdgeKindsBuilder {
    fn build(self) -> Option<Vec<EdgeKind>> {
        Some(self.0)
    }
}

/// Kinds of nodes
#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Graph,
    Node,
    Edge,

    /// User variable
    Custom(String),
}

impl std::fmt::Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Graph => "graph",
                Self::Node => "node",
                Self::Edge => "edge",
                Self::Custom(s) => s.as_str(),
            }
        )
    }
}

impl EdgeKindsBuilder {
    pub fn node(mut self, name: impl Into<NodeKind>, attributes: Option<Vec<Attribute>>) -> Self {
        self.0.push(EdgeKind::Node(name.into(), attributes));
        self
    }

    pub fn edge(mut self, edge: impl Into<Edge>) -> Self {
        self.0.push(EdgeKind::Edge(Box::new(edge.into())));
        self
    }

    pub fn subgraph(mut self, body: &[EdgeKind]) -> Self {
        self.0.push(EdgeKind::SubGraph(body.to_vec()));
        self
    }

    pub fn comment(mut self, text: impl Into<String>) -> Self {
        self.0.push(EdgeKind::Comment(text.into()));
        self
    }

    pub fn graph(mut self, graph: impl Into<GraphKind>) -> Self {
        self.0.push(EdgeKind::Graph(graph.into()));

        self
    }

    pub fn digraph(mut self, graph: impl Into<GraphKind>) -> Self {
        self.0.push(EdgeKind::Graph(graph.into()));

        self
    }
}

/// Builder for a singular `EdgeKind`
#[derive(Default)]
pub struct EdgeKindBuilder(Option<EdgeKind>);
impl_into_underlying!(EdgeKind, EdgeKindBuilder);

impl Builder<EdgeKind> for EdgeKindBuilder {
    fn build(self) -> Option<EdgeKind> {
        self.0
    }
}

impl EdgeKindBuilder {
    pub fn node(mut self, kind: NodeKind, attributes: Option<Vec<Attribute>>) -> Self {
        self.0 = Some(EdgeKind::Node(kind, attributes));
        self
    }

    pub fn edge(mut self, edge: impl Into<Edge>) -> Self {
        self.0 = Some(EdgeKind::Edge(Box::new(edge.into())));
        self
    }

    pub fn subgraph(mut self, body: &[EdgeKind]) -> Self {
        self.0 = Some(EdgeKind::SubGraph(body.to_vec()));
        self
    }

    pub fn comment(mut self, text: impl Into<String>) -> Self {
        self.0 = Some(EdgeKind::Comment(text.into()));
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    left: EdgeKind,
    right: EdgeKind,
    attributes: Option<Vec<Attribute>>,
}

/// Builder for `Edge`s
#[derive(Default)]
pub struct EdgeBuilder {
    left: Option<EdgeKind>,
    right: Option<EdgeKind>,
    attributes: Option<Vec<Attribute>>,
}
impl_into_underlying!(Edge, EdgeBuilder);

impl Builder<Edge> for EdgeBuilder {
    fn build(self) -> Option<Edge> {
        Some(Edge {
            left: self.left?,
            right: self.right?,
            attributes: self.attributes,
        })
    }
}

impl EdgeBuilder {
    pub fn left(mut self, left: impl Into<EdgeKind>) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn right(mut self, right: impl Into<EdgeKind>) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn attributes(mut self, attributes: impl Into<Vec<Attribute>>) -> Self {
        self.attributes = Some(attributes.into());
        self
    }
}
