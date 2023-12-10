use super::super::super::super::math::distance::DistanceMetric;
use super::super::super::super::math::neighbors::neighbor::Neighbor;
use super::super::super::super::math::neighbors::search::NeighborSearch;
use super::super::super::super::math::number::Float;
use super::super::super::super::math::point::Point;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::marker::PhantomData;

/// Struct representing linear search algorithm for neighbor search.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
/// * `P` - The type of points used in the neighbor search algorithm.
#[derive(Debug)]
pub struct LinearSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    points: Cow<'a, [P]>,
    metric: &'a DistanceMetric,
    _marker: PhantomData<F>,
}

impl<'a, F, P> LinearSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    /// Creates a new `LinearSearch` instance.
    ///
    /// # Arguments
    /// * `points` - The reference of a dataset of points.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `LinearSearch` instance.
    #[allow(unused)]
    #[must_use]
    pub fn new(points: &'a Vec<P>, metric: &'a DistanceMetric) -> Self {
        Self {
            points: Cow::Borrowed(points),
            metric,
            _marker: PhantomData,
        }
    }
}

impl<'a, F, P> NeighborSearch<F, P> for LinearSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn search(&self, query: &P, k: usize) -> Vec<Neighbor<F>> {
        if k == 0 {
            return Vec::new();
        }

        let mut neighbors: Vec<Neighbor<F>> = self
            .points
            .iter()
            .enumerate()
            .map(|(index, point)| {
                let distance = self.metric.measure(point, query);
                Neighbor::new(index, distance)
            })
            .collect();

        neighbors.sort_unstable_by(|neighbor1, neighbor2| {
            neighbor1
                .distance
                .partial_cmp(&neighbor2.distance)
                .unwrap_or(Ordering::Equal)
        });
        neighbors.truncate(k);
        neighbors
    }

    #[must_use]
    fn search_nearest(&self, query: &P) -> Option<Neighbor<F>> {
        self.search(query, 1).pop()
    }

    #[must_use]
    fn search_radius(&self, query: &P, radius: F) -> Vec<Neighbor<F>> {
        if radius < F::zero() {
            return Vec::new();
        }

        let mut neighbors: Vec<_> = self
            .points
            .iter()
            .enumerate()
            .filter_map(|(index, point)| {
                let distance = self.metric.measure(point, query);
                if distance <= radius {
                    Some(Neighbor::new(index, distance))
                } else {
                    None
                }
            })
            .collect();

        neighbors.sort_unstable_by(|neighbor1, neighbor2| {
            neighbor1
                .distance
                .partial_cmp(&neighbor2.distance)
                .unwrap_or(Ordering::Equal)
        });
        neighbors
    }
}
