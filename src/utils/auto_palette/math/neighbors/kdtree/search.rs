use super::super::super::super::math::distance::DistanceMetric;
use super::super::super::super::math::neighbors::kdtree::node::KDNode;
use super::super::super::super::math::neighbors::neighbor::Neighbor;
use super::super::super::super::math::neighbors::search::NeighborSearch;
use super::super::super::super::math::number::Float;
use super::super::super::super::math::point::Point;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::marker::PhantomData;

/// Struct representing kd-tree search algorithm for neighbor search.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
/// * `P` - The type of points used in the neighbor search algorithm.
#[derive(Debug)]
pub struct KDTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    root: Option<Box<KDNode>>,
    points: Cow<'a, [P]>,
    metric: &'a DistanceMetric,
    _marker: PhantomData<F>,
}

impl<'a, F, P> KDTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F> + 'a,
{
    /// Creates a new `KDTreeSearch` instance.
    ///
    /// # Arguments
    /// * `points` - The reference of a dataset of points.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `KDTreeSearch` instance.
    #[must_use]
    pub fn new(points: &'a [P], metric: &'a DistanceMetric) -> Self {
        let mut indices: Vec<usize> = (0..points.len()).collect();
        let root = Self::build_node(points, &mut indices, 0);

        Self {
            root: root.map(Box::new),
            points: Cow::Borrowed(points),
            metric,
            _marker: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    fn partition_by_key<V, T>(slice: &mut [T], value_fn: &V) -> usize
    where
        T: Ord,
        V: Fn(&T) -> F,
    {
        let pivot = slice.len() / 2;
        let pivot_value = value_fn(&slice[pivot]);

        let mut left = 0;
        let mut right = slice.len() - 1;
        while left <= right {
            while value_fn(&slice[left]) < pivot_value {
                left += 1;
            }
            while value_fn(&slice[right]) > pivot_value {
                right -= 1;
            }

            if left <= right {
                slice.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
        left - 1
    }

    #[inline]
    #[must_use]
    fn find_nth_index<T, V>(slice: &mut [T], n: usize, value_fn: V) -> usize
    where
        T: Ord,
        V: Fn(&T) -> F,
    {
        if slice.len() <= 1 {
            return 0;
        }

        let pivot_index = Self::partition_by_key(slice, &value_fn);
        match n.cmp(&pivot_index) {
            Ordering::Less => Self::find_nth_index(&mut slice[..pivot_index], n, value_fn),
            Ordering::Greater => {
                let index = Self::find_nth_index(
                    &mut slice[pivot_index + 1..],
                    n - pivot_index - 1,
                    value_fn,
                );
                index + pivot_index + 1
            }
            _ => pivot_index,
        }
    }

    #[inline]
    #[must_use]
    fn build_node(points: &[P], indices: &mut [usize], depth: usize) -> Option<KDNode> {
        if indices.is_empty() {
            return None;
        }

        let axis = depth % points[0].dimension();
        let median = indices.len() / 2;
        let median_index = Self::find_nth_index(indices, median, |&index: &usize| {
            let point = &points[index];
            point[axis]
        });

        let node = KDNode::new(
            indices[median_index],
            axis,
            Self::build_node(points, &mut indices[..median], depth + 1),
            Self::build_node(points, &mut indices[median + 1..], depth + 1),
        );
        Some(node)
    }

    #[inline]
    fn search_recursively(
        &self,
        root: &Option<Box<KDNode>>,
        query: &P,
        k: usize,
        neighbors: &mut Vec<Neighbor<F>>,
    ) {
        let Some(ref node) = root else {
            return;
        };

        let point = &self.points[node.index];
        let distance = self.metric.measure(point, query);
        let neighbor = Neighbor::new(node.index, distance);
        if neighbors.len() < k {
            neighbors.push(neighbor);
            neighbors.sort_unstable_by(|neighbor1, neighbor2| {
                neighbor1
                    .distance
                    .partial_cmp(&neighbor2.distance)
                    .unwrap_or(Ordering::Equal)
            });
        } else if distance < neighbors[k - 1].distance {
            neighbors.pop();
            neighbors.push(neighbor);
            neighbors.sort_unstable_by(|neighbor1, neighbor2| {
                neighbor1
                    .distance
                    .partial_cmp(&neighbor2.distance)
                    .unwrap_or(Ordering::Equal)
            });
        }

        if node.is_leaf() {
            return;
        }

        let delta = query[node.axis] - point[node.axis];
        if neighbors.len() < k || delta.abs() <= neighbors[k - 1].distance {
            self.search_recursively(node.left(), query, k, neighbors);
            self.search_recursively(node.right(), query, k, neighbors);
        } else if delta < F::zero() {
            self.search_recursively(node.left(), query, k, neighbors);
        } else {
            self.search_recursively(node.right(), query, k, neighbors);
        }
    }

    #[inline]
    #[must_use]
    fn search_nearest_recursively(
        &self,
        root: &Option<Box<KDNode>>,
        query: &P,
        best_neighbor: Option<Neighbor<F>>,
    ) -> Option<Neighbor<F>> {
        let Some(ref node) = root else {
            return best_neighbor;
        };

        let point = &self.points[node.index];
        let distance = self.metric.measure(point, query);
        let neighbor = Neighbor::new(node.index, distance);

        let best_distance = best_neighbor.map(|n| n.distance).unwrap_or(F::max_value());
        if distance >= best_distance {
            return best_neighbor;
        }

        let nearest = Some(neighbor);
        if node.is_leaf() {
            return nearest;
        }

        let delta = query[node.axis] - point[node.axis];
        let (primary, secondary) = if delta < F::zero() {
            (node.left(), node.right())
        } else {
            (node.right(), node.left())
        };

        let nearest = self.search_nearest_recursively(primary, query, nearest);
        let best_distance = nearest.map(|n| n.distance).unwrap_or(F::max_value());
        if delta.abs() < best_distance {
            self.search_nearest_recursively(secondary, query, nearest)
        } else {
            nearest
        }
    }

    #[inline]
    fn search_radius_recursively(
        &self,
        root: &Option<Box<KDNode>>,
        query: &P,
        radius: F,
        neighbors: &mut Vec<Neighbor<F>>,
    ) {
        let Some(ref node) = root else {
            return;
        };

        let point = self.points[node.index];
        let distance = self.metric.measure(&point, query);
        if distance <= radius {
            neighbors.push(Neighbor::new(node.index, distance));
        }

        let delta = query[node.axis] - point[node.axis];
        if delta.abs() <= radius {
            self.search_radius_recursively(node.left(), query, radius, neighbors);
            self.search_radius_recursively(node.right(), query, radius, neighbors);
        } else if delta < F::zero() {
            self.search_radius_recursively(node.left(), query, radius, neighbors);
        } else {
            self.search_radius_recursively(node.right(), query, radius, neighbors);
        }
    }
}

impl<'a, F, P> NeighborSearch<F, P> for KDTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn search(&self, query: &P, k: usize) -> Vec<Neighbor<F>> {
        if k == 0 {
            return Vec::new();
        }

        let mut neighbors = Vec::new();
        self.search_recursively(&self.root, query, k, &mut neighbors);
        neighbors.sort_unstable_by(|neighbor1, neighbor2| {
            neighbor1
                .distance
                .partial_cmp(&neighbor2.distance)
                .unwrap_or(Ordering::Equal)
        });
        neighbors.into_iter().take(k).collect()
    }

    #[must_use]
    fn search_nearest(&self, query: &P) -> Option<Neighbor<F>> {
        self.search_nearest_recursively(&self.root, query, None)
    }

    #[must_use]
    fn search_radius(&self, query: &P, radius: F) -> Vec<Neighbor<F>> {
        if radius < F::zero() {
            return Vec::new();
        }

        let mut neighbors = Vec::with_capacity(32);
        self.search_radius_recursively(&self.root, query, radius, &mut neighbors);
        neighbors
    }
}