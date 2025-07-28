use mago_syntax::ast::Node;

/// A precomputed tree node for the AST.
///
/// `AstNode` wraps a [`Node`] from the AST and precomputes its children into a vector of [`PreComputedNode`]
/// structures. This avoids multiple calls to [`Node::children()`] during linting, thereby optimizing
/// traversal performance.
#[derive(Debug)]
pub(crate) struct PreComputedNode<'a> {
    /// The wrapped AST node.
    pub node: Node<'a>,
    /// The precomputed child nodes.
    pub children: Vec<PreComputedNode<'a>>,
}

impl<'a> From<Node<'a>> for PreComputedNode<'a> {
    /// Recursively converts a [`Node`] into an [`PreComputedNode`], precomputing its children.
    ///
    /// # Parameters
    ///
    /// - `node`: The AST node to be converted.
    ///
    /// # Returns
    ///
    /// An [`PreComputedNode`] representing the given node and its descendants.
    fn from(node: Node<'a>) -> Self {
        let node_children = node.children();
        let mut children = Vec::with_capacity(node_children.len());
        for child in node_children {
            children.push(PreComputedNode::from(child));
        }

        Self { node, children }
    }
}
