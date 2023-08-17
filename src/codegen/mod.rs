mod tests;

/// Graffy's formatter to turn DOT tree -> DOT source code.
pub mod formatter;

/// Builder trait for all builders of `T`
pub trait Builder<T>: Into<T> + Default {
    /// Consume `self` and try to build `T`, may fail
    fn build(self) -> Option<T>;
}

/// Trait that gets the builder for a type
pub trait Buildable<B: Builder<Self>>
where
    Self: Sized,
{
    /// Static function that gets a builder for `Self`
    fn builder() -> B;
}

macro_rules! impl_builder {
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

        impl Buildable<$builder> for $kind {
            fn builder() -> $builder {
                <$builder>::default()
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graph {
    name: Option<String>,
    edges: Vec<EdgeKind>,
    is_strict: bool,
    attributes: Vec<Attribute>,
}

/// Is the Graph a digraph or a graph?
#[derive(Debug, Clone, PartialEq)]
pub enum GraphKind {
    Undirected(Graph),
    Directed(Graph),
}

/// Get a `Graph` from a `GraphKind` (regardless of the graph's direction)
impl Into<Graph> for GraphKind {
    fn into(self) -> Graph {
        match self {
            Self::Directed(graph) | Self::Undirected(graph) => graph,
        }
    }
}

/// Builder for a `GraphKind`
#[derive(Default)]
pub struct GraphBuilder {
    name: Option<String>,
    edges: Vec<EdgeKind>,
    is_strict: Option<bool>,
    attributes: Vec<Attribute>,
    is_directed: bool,
}
impl_builder!(GraphKind, GraphBuilder);

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

    pub fn edges(mut self, edges: impl Into<Vec<EdgeKind>>) -> Self {
        self.edges = edges.into();
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

/// Kinds of shapes
#[derive(Debug, Clone, PartialEq)]
pub enum ShapeKind {
    /// `ellipse`
    Ellipse,

    /// `box`
    Box,

    /// `circle`
    Circle,

    /// `diamond`
    Diamond,
}

impl std::fmt::Display for ShapeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ellipse => "ellipse",
                Self::Box => "box",
                Self::Circle => "circle",
                Self::Diamond => "diamond",
            }
        )
    }
}

/// Kinds of colors
#[derive(Debug, Clone, PartialEq)]
pub enum ColorKind {
    /// `blue`
    Blue,
}

impl std::fmt::Display for ColorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Blue => "blue",
            }
        )
    }
}

/// Kinds of edge styles
#[derive(Debug, Clone, PartialEq)]
pub enum StyleKind {
    Solid,
    Dotted,
    Dashed,
}

impl std::fmt::Display for StyleKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Solid => write!(f, "solid"),
            Self::Dotted => write!(f, "dotted"),
            Self::Dashed => write!(f, "dashed"),
        }
    }
}

/// Kinds of edge directions
#[derive(Debug, Clone, PartialEq)]
pub enum DirectionKind {
    /// `forward`
    Forward,

    /// `back`
    Back,
}

impl std::fmt::Display for DirectionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionKind::Forward => write!(f, "forward"),
            DirectionKind::Back => write!(f, "back"),
        }
    }
}

/// Kinds of graph layouts
#[derive(Debug, Clone, PartialEq)]
pub enum RankDirectionKind {
    /// `TB`: for top-to-bottom
    TopBottom,

    /// 'LR': for left-to-right
    LeftRight,
}

impl std::fmt::Display for RankDirectionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RankDirectionKind::TopBottom => write!(f, "TB"),
            RankDirectionKind::LeftRight => write!(f, "LR"),
        }
    }
}

/// Kinds of styles for edge curves
#[derive(Debug, Clone, PartialEq)]
pub enum CurveStyleKind {
    /// `true`
    True,

    /// `false`
    False,

    /// `ortho`
    Ortho,
}

impl std::fmt::Display for CurveStyleKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurveStyleKind::True => write!(f, "true"),
            CurveStyleKind::False => write!(f, "false"),
            CurveStyleKind::Ortho => write!(f, "ortho"),
        }
    }
}

/// Attributes for edges
#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    /// `fontname`
    FontName(String),

    /// `label`
    Label(String),

    /// `shape`: Specifies the shape of the node (e.g., `box`, `ellipse`, `circle`, `diamond`, etc.).
    Shape(ShapeKind),

    /// `color`: Sets the color of the nodes border/edge.
    Color(ColorKind),

    /// `fillcolor`: Sets the background color of the node.
    FillColor(ColorKind),

    /// `tooltip`: Tooltip attached to the node.
    Tooltip(String),

    /// `URL`: URL attached to the node.
    URL(String),

    /// `style`: Sets the line style of the edge(s) (e.g., `dotted`, `dashed`, `solid`, etc.).
    Style(StyleKind),

    /// `dir`: Sets the direction of the edge (i.e. `forward`, `back`, or `none`).
    Direction(Option<DirectionKind>),

    /// `fontsize`: Sets the font size for the edge's label.
    FontSize(usize),

    /// `bgcolor`: Sets the background color of the graph.
    BgColor(ColorKind),

    /// `rankdir`: Sets the direction of graph layout (`TB` for top-bottom, `LR` for left-right, etc).
    RankDirection(RankDirectionKind),

    /// `splines`: Sets the style of the edge's line.
    CurveStyle(CurveStyleKind),
}

impl Into<Vec<Attribute>> for Attribute {
    fn into(self) -> Vec<Attribute> {
        vec![self]
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FontName(name) => format!(r#"fontname="{name}""#),
                Self::Label(label) => format!(r#"label="{label}""#),
                Self::Shape(kind) => format!("shape={kind}"),
                Self::Color(kind) => format!("color={kind}"),
                Self::FillColor(kind) => format!("fillcolor={kind}"),
                Self::Tooltip(text) => format!(r#"tooltip="{text}""#),
                Self::URL(url) => format!(r#"URL="{url}""#),
                Self::Style(kind) => format!("style={kind}"),
                Self::Direction(Some(kind)) => format!("dir={kind}"),
                Self::Direction(None) => format!("dir=none"),
                Self::FontSize(size) => format!(r#"fontsize="{size}""#),
                Self::BgColor(color_kind) => format!("bgcolor={color_kind}"),
                Self::RankDirection(kind) => format!("rankdir={kind}"),
                Self::CurveStyle(kind) => {
                    format!("splines={}", kind)
                }
            }
        )
    }
}

/// Builder for multiple attributes
#[derive(Default)]
pub struct AttributesBuilder(Vec<Attribute>);
impl_builder!(Vec<Attribute>, AttributesBuilder);

impl AsRef<[Attribute]> for AttributesBuilder {
    fn as_ref(&self) -> &[Attribute] {
        &self.0
    }
}

impl Builder<Vec<Attribute>> for AttributesBuilder {
    fn build(self) -> Option<Vec<Attribute>> {
        Some(self.0)
    }
}

impl AttributesBuilder {
    pub fn font_name(mut self, f: impl Into<String>) -> Self {
        self.0.push(Attribute::FontName(f.into()));
        self
    }

    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.0.push(Attribute::Label(l.into()));
        self
    }

    pub fn shape(mut self, shape: impl Into<ShapeKind>) -> Self {
        self.0.push(Attribute::Shape(shape.into()));
        self
    }

    pub fn color(mut self, color: impl Into<ColorKind>) -> Self {
        self.0.push(Attribute::Color(color.into()));
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    kind: NodeKind,
    attributes: Option<Vec<Attribute>>,
}

/// Builder for `Node`s
#[derive(Default)]
pub struct NodeBuilder {
    kind: Option<NodeKind>,
    attributes: Option<Vec<Attribute>>,
}
impl_builder!(Node, NodeBuilder);

impl Builder<Node> for NodeBuilder {
    fn build(self) -> Option<Node> {
        Some(Node {
            kind: self.kind?,
            attributes: self.attributes,
        })
    }
}

impl NodeBuilder {
    pub fn kind(mut self, kind: impl Into<NodeKind>) -> Self {
        self.kind = Some(kind.into());
        self
    }

    pub fn attributes(mut self, attributes: impl AsRef<[Attribute]>) -> Self {
        self.attributes = Some(attributes.as_ref().to_vec());
        self
    }
}

/// Data for a subgraph
#[derive(Debug, Clone, PartialEq)]
pub struct SubGraph {
    name: Option<String>,
    body: Vec<EdgeKind>,
}

/// Builder for `SubGraph`
#[derive(Default)]
pub struct SubGraphBuilder {
    name: Option<String>,
    body: Option<Vec<EdgeKind>>,
}

impl Builder<SubGraph> for SubGraphBuilder {
    fn build(self) -> Option<SubGraph> {
        Some(SubGraph {
            name: self.name,
            body: self.body?,
        })
    }
}
impl_builder!(SubGraph, SubGraphBuilder);

impl SubGraphBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn body(mut self, body: impl Into<Vec<EdgeKind>>) -> Self {
        self.body = Some(body.into());
        self
    }
}

/// Kinds of `Edge`s and their data
#[derive(Debug, Clone, PartialEq)]
pub enum EdgeKind {
    /// `Node(node)` -> `node.kind [node.attributes]`
    Node(Node),

    /// `Edge(edge)` -> `edge.left -> edge.right [edge.attributes]`
    Edge(Box<Edge>),

    /// `SubGraph(body)` -> `subgraph { body }`
    SubGraph(SubGraph),

    /// `Comment(text)` -> `// text`
    Comment(String),

    Graph(GraphKind),
}

/// Builder for edge kinds
#[derive(Default)]
pub struct EdgeKindsBuilder(Vec<EdgeKind>);
impl_builder!(Vec<EdgeKind>, EdgeKindsBuilder);

impl AsRef<[EdgeKind]> for EdgeKindsBuilder {
    fn as_ref(&self) -> &[EdgeKind] {
        &self.0
    }
}

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

impl<T: AsRef<str>> From<T> for NodeKind {
    fn from(v: T) -> Self {
        Self::Custom(v.as_ref().into())
    }
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
                Self::Custom(s) => &s,
            }
        )
    }
}

impl EdgeKindsBuilder {
    pub fn node(
        mut self,
        name: impl Into<NodeKind>,
        attributes: impl Into<Vec<Attribute>>,
    ) -> Self {
        self.0.push(EdgeKind::Node(Node {
            kind: name.into(),
            attributes: Some(attributes.into()),
        }));
        self
    }

    pub fn edge(mut self, edge: impl Into<Edge>) -> Self {
        self.0.push(EdgeKind::Edge(Box::new(edge.into())));
        self
    }

    pub fn subgraph(mut self, subgraph: impl Into<SubGraph>) -> Self {
        self.0.push(EdgeKind::SubGraph(subgraph.into()));
        self
    }

    pub fn comment(mut self, text: impl Into<String>) -> Self {
        self.0.push(EdgeKind::Comment(text.into()));
        self
    }

    pub fn graph(mut self, graph: impl Into<GraphKind>) -> Self {
        self.0
            .push(EdgeKind::Graph(GraphKind::Undirected(match graph.into() {
                GraphKind::Directed(graph) | GraphKind::Undirected(graph) => graph,
            })));

        self
    }

    pub fn digraph(mut self, graph: impl Into<GraphKind>) -> Self {
        self.0
            .push(EdgeKind::Graph(GraphKind::Directed(match graph.into() {
                GraphKind::Directed(graph) | GraphKind::Undirected(graph) => graph,
            })));

        self
    }
}

/// Builder for a singular `EdgeKind`
#[derive(Default)]
pub struct EdgeKindBuilder(Option<EdgeKind>);
impl_builder!(EdgeKind, EdgeKindBuilder);

impl Builder<EdgeKind> for EdgeKindBuilder {
    fn build(self) -> Option<EdgeKind> {
        self.0
    }
}

impl EdgeKindBuilder {
    pub fn node(mut self, kind: NodeKind, attributes: Option<Vec<Attribute>>) -> Self {
        self.0 = Some(EdgeKind::Node(Node { kind, attributes }));
        self
    }

    pub fn edge(mut self, edge: impl Into<Edge>) -> Self {
        self.0 = Some(EdgeKind::Edge(Box::new(edge.into())));
        self
    }

    pub fn subgraph(mut self, subgraph: impl Into<SubGraph>) -> Self {
        self.0 = Some(EdgeKind::SubGraph(subgraph.into()));
        self
    }

    pub fn comment(mut self, text: impl Into<String>) -> Self {
        self.0 = Some(EdgeKind::Comment(text.into()));
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    left: NodeKind,
    right: NodeKind,
    attributes: Option<Vec<Attribute>>,
}

/// Builder for `Edge`s
#[derive(Default)]
pub struct EdgeBuilder {
    left: Option<NodeKind>,
    right: Option<NodeKind>,
    attributes: Option<Vec<Attribute>>,
}
impl_builder!(Edge, EdgeBuilder);

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
    pub fn left(mut self, left: impl Into<NodeKind>) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn right(mut self, right: impl Into<NodeKind>) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn attributes(mut self, attributes: impl Into<Vec<Attribute>>) -> Self {
        self.attributes = Some(attributes.into());
        self
    }
}
