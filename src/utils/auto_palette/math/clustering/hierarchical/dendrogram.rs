use super::super::super::super::math::clustering::cmp::Priority;
use super::super::super::super::math::clustering::hierarchical::node::Node;
use super::super::super::super::number::Float;
use std::collections::BinaryHeap;

/// Struct representing a dendrogram.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
#[derive(Debug)]
pub struct Dendrogram<F>
where
    F: Float,
{
    nodes: Vec<Node<F>>,
}

impl<F> Dendrogram<F>
where
    F: Float,
{
    /// Creates a new `Dendrogram` instance with the given capacity.
    ///
    /// # Arguments
    /// * `capacity` - The capacity of the new dendrogram.
    ///
    /// # Returns
    /// A new `Dendrogram` instance.

    pub fn new(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of nodes in this dendrogram.
    ///
    /// # Returns
    /// The number of nodes in this dendrogram.

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns a reference to the nodes of this dendrogram.
    ///
    /// # Returns
    /// A reference to the nodes of this dendrogram.

    pub fn nodes(&self) -> &[Node<F>] {
        &self.nodes
    }

    /// Pushes a new node to this dendrogram.
    ///
    /// # Arguments
    /// * `node` - The node to push.
    #[inline]
    pub fn push(&mut self, node: Node<F>) {
        assert!(self.len() < self.nodes.capacity());
        self.nodes.push(node);
    }

    /// Partitions this dendrogram into `n` clusters.
    ///
    /// # Arguments
    /// * `n` - The number of clusters to partition this dendrogram into.
    ///
    /// # Returns
    /// A vector of nodes representing the clusters.

    pub fn partition(&self, n: usize) -> Vec<Node<F>> {
        let mut heap = BinaryHeap::new();
        if let Some(node) = self.nodes.last() {
            heap.push(Priority::new(node, node.distance));
        }

        let mut membership = Vec::with_capacity(n);
        while membership.len() < n {
            if heap.len() + membership.len() >= n {
                membership.extend(heap.iter().map(|&Priority(node, _)| node.clone()));
                membership.truncate(n);
                break;
            }

            let Some(Priority(node, _)) = heap.pop() else {
                break;
            };

            if let Some(node1) = node.node1 {
                let node = &self.nodes[node1];
                heap.push(Priority::new(node, node.distance));
            } else {
                membership.push(node.clone());
            }

            if let Some(node2) = node.node2 {
                let node = &self.nodes[node2];
                heap.push(Priority::new(node, node.distance));
            } else {
                membership.push(node.clone());
            }
        }
        membership
    }
}
