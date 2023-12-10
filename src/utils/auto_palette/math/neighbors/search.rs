use super::super::super::math::neighbors::neighbor::Neighbor;
use super::super::super::math::number::Float;
use super::super::super::math::point::Point;

/// Trait representing neighbor search algorithms.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
/// * `P` - The type of points used in the neighbor search algorithm.
pub trait NeighborSearch<F, P>
where
    F: Float,
    P: Point<F>,
{
    /// Searches for the k-nearest neighbors of the given point.
    ///
    /// # Arguments
    /// * `query` - The reference point of the neighbors are searched.
    /// * `k` - The number of nearest neighbors.
    ///
    /// # Returns
    /// A `Vec` of the k-nearest neighbors.
    #[must_use]
    fn search(&self, query: &P, k: usize) -> Vec<Neighbor<F>>;

    /// Search for the nearest neighbor of the given point.
    ///
    /// # Arguments
    /// * `query` - The reference point of the neighbor is searched.
    ///
    /// # Returns
    /// An `Option` of the nearest neighbor.
    #[must_use]
    fn search_nearest(&self, query: &P) -> Option<Neighbor<F>>;

    /// Searches for all neighbors within the given radius of a given point.
    ///
    /// # Arguments
    /// * `query` - The reference point of the neighbors are searched.
    /// * `radius` - The radius within neighbors should be searched..
    ///
    /// # Returns
    /// A `Vec` of all neighbors within the given radius.
    #[must_use]
    fn search_radius(&self, query: &P, radius: F) -> Vec<Neighbor<F>>;
}
