/// Struct representing a node of kd-tree.
#[derive(Debug)]
pub struct KDNode {
    /// The index of a point in the points.
    pub index: usize,

    /// The axis of the split.
    pub axis: usize,

    left: Option<Box<KDNode>>,
    right: Option<Box<KDNode>>,
}

impl KDNode {
    /// Creates a new `KDNode` instance.
    ///
    /// # Arguments
    /// * `index` - The index of a point.
    /// * `axis` - The axis of the split.
    /// * `left` - The left child node.
    /// * `right` - The right child node.
    ///
    /// # Returns
    /// A new `KDNode` instance.
    #[must_use]
    pub fn new(index: usize, axis: usize, left: Option<KDNode>, right: Option<KDNode>) -> Self {
        Self {
            index,
            axis,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }

    /// Returns a reference to the left child node.
    ///
    /// # Returns
    /// A reference to the left child node.
    #[must_use]
    pub fn left(&self) -> &Option<Box<KDNode>> {
        &self.left
    }

    /// Returns a reference to the right child node.
    ///
    /// # Returns
    /// A reference to the right child node.
    #[must_use]
    pub fn right(&self) -> &Option<Box<KDNode>> {
        &self.right
    }

    /// Checks whether this node is a leaf node.
    ///
    /// # Returns
    /// `true` if this node is a leaf node, otherwise `false`.
    #[must_use]
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}