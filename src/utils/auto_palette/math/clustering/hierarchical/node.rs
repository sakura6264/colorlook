use super::super::super::super::number::Float;

/// Struct representing a node in a dendrogram.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
#[derive(Debug, Clone, PartialEq)]
pub struct Node<F>
where
    F: Float,
{
    /// The label of this node.
    pub label: usize,

    /// The label of the 1st child node.
    pub node1: Option<usize>,

    /// The label of the 2nd child node.
    pub node2: Option<usize>,

    /// The distance between the 1st and 2nd child node.
    pub distance: F,
}

impl<F> Node<F>
where
    F: Float,
{
    /// Creates a new `Node` instance.
    ///
    /// # Arguments
    /// * `label` - The label of this node.
    /// * `node1` - The label of the 1st child node.
    /// * `node2` - The label of the 2nd child node.
    /// * `distance` - The distance between the 1st and 2nd child node.
    ///
    /// # Returns
    /// A new `Node` instance.
    #[inline]
    #[must_use]
    pub fn new(label: usize, node1: Option<usize>, node2: Option<usize>, distance: F) -> Self {
        Self {
            label,
            node1,
            node2,
            distance,
        }
    }
}
