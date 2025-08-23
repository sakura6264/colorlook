use super::super::super::super::math::clustering::cmp::Priority;
use super::super::super::super::math::clustering::hierarchical::dendrogram::Dendrogram;
use super::super::super::super::math::clustering::hierarchical::linkage::{Linkage, SingleLinkage};
use super::super::super::super::math::clustering::hierarchical::node::Node;
use super::super::super::super::number::Float;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::marker::PhantomData;

/// Struct representing a hierarchical clustering algorithm.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
#[derive(Debug, PartialEq)]
pub struct HierarchicalClustering<F>
where
    F: Float,
{
    _marker: PhantomData<F>,
}

impl<'a, F> HierarchicalClustering<F>
where
    F: Float,
{
    /// Creates a new `HierarchicalClustering` instance.
    ///
    /// # Returns
    /// A new `HierarchicalClustering` instance.

    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Fits the hierarchical clustering algorithm to the given dataset.
    ///
    /// # Arguments
    /// * `dataset` - The dataset to fit the algorithm to.
    /// * `distance_fn` - The distance function to use.
    ///
    /// # Returns
    /// A dendrogram representing the clustering.
    ///
    /// # Type Parameters
    /// * `T` - The type of the elements in the dataset.
    /// * `DF` - The type of the distance function.
    #[allow(unused)]

    pub fn fit<T, DF>(&self, dataset: &'a [T], distance_fn: &'a DF) -> Dendrogram<F>
    where
        DF: Fn(&T, &T) -> F,
    {
        self.fit_with_linkage(dataset, &mut SingleLinkage::new(dataset, distance_fn))
    }

    /// Fits the hierarchical clustering algorithm with the given linkage to the given dataset.
    ///
    /// # Arguments
    /// * `dataset` - The dataset to fit the algorithm to.
    /// * `linkage` - The linkage to use.
    ///
    /// # Returns
    /// A dendrogram representing the clustering.
    ///
    /// # Type Parameters
    /// * `T` - The type of the elements in the dataset.

    pub fn fit_with_linkage<T>(
        &self,
        dataset: &'a [T],
        linkage: &mut impl Linkage<F>,
    ) -> Dendrogram<F> {
        let n_dataset = dataset.len();
        let mut dendrogram = Dendrogram::new(n_dataset * 2 - 1);
        dataset.iter().enumerate().for_each(|(i, _)| {
            let node = Node::new(i, None, None, F::zero());
            dendrogram.push(node);
        });

        let mut heap = BinaryHeap::new();
        for i in 0..dendrogram.len() {
            for j in (i + 1)..dendrogram.len() {
                let distance = linkage.distance(i, j);
                let priority = Priority::new((i, j), distance);
                heap.push(Reverse(priority));
            }
        }

        let mut inactive_nodes = HashSet::new();
        while let Some(Reverse(Priority(pair, distance))) = heap.pop() {
            let (label1, label2) = pair;
            if inactive_nodes.contains(&label1) || inactive_nodes.contains(&label2) {
                continue;
            }

            let label = linkage.merge(label1, label2);
            inactive_nodes.insert(label1);
            inactive_nodes.insert(label2);

            let merged_node = Node::new(label, Some(label1), Some(label2), distance);
            dendrogram.push(merged_node);

            for i in 0..label {
                if inactive_nodes.contains(&i) {
                    continue;
                }

                let distance = linkage.distance(i, label);
                let priority = Priority::new((i, label), distance);
                heap.push(Reverse(priority));
            }
        }
        dendrogram
    }
}
