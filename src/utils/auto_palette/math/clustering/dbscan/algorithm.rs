use super::super::super::super::math::clustering::algorithm::ClusteringAlgorithm;
use super::super::super::super::math::clustering::cluster::Cluster;
use super::super::super::super::math::clustering::dbscan::label::Label;
use super::super::super::super::math::distance::DistanceMetric;
use super::super::super::super::math::neighbors::kdtree::search::KDTreeSearch;
use super::super::super::super::math::neighbors::neighbor::Neighbor;
use super::super::super::super::math::neighbors::search::NeighborSearch;
use super::super::super::super::math::number::Float;
use super::super::super::super::math::point::Point;
use std::collections::{HashMap, HashSet, VecDeque};

/// Struct representing DBSCAN clustering algorithm.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub struct DBSCAN<'a, F>
where
    F: Float,
{
    min_samples: usize,
    epsilon: F,
    metric: &'a DistanceMetric,
}

impl<'a, F> DBSCAN<'a, F>
where
    F: Float,
{
    /// Creates a new `DBSCAN` instance.
    ///
    /// # Arguments
    /// * `min_samples` - The minimum number of points.
    /// * `epsilon` - The maximum distance between two points.
    /// * `metric` - The distance metric.
    ///
    /// # Returns
    /// A new `DBSCAN` instance.
    #[must_use]
    pub fn new(min_samples: usize, epsilon: F, metric: &'a DistanceMetric) -> Self {
        Self {
            min_samples,
            epsilon,
            metric,
        }
    }

    fn expand_cluster<P, N>(
        &self,
        cluster_id: usize,
        points: &[P],
        ns: &N,
        neighbors: &[Neighbor<F>],
        labels: &mut [Label],
    ) where
        P: Point<F>,
        N: NeighborSearch<F, P>,
    {
        let mut queue = VecDeque::new();
        queue.extend(neighbors.iter().map(|n| n.index));
        while let Some(current_index) = queue.pop_front() {
            if labels[current_index].is_assigned() {
                continue;
            }

            if labels[current_index].is_outlier() {
                labels[current_index] = Label::Assigned(cluster_id);
                continue;
            }

            labels[current_index] = Label::Assigned(cluster_id);

            let point = points[current_index];
            let secondary_neighbors = ns.search_radius(&point, self.epsilon);
            if secondary_neighbors.len() < self.min_samples {
                continue;
            }

            for secondary_neighbor in secondary_neighbors.into_iter() {
                let secondary_index = secondary_neighbor.index;
                match labels[secondary_index] {
                    Label::Undefined => {
                        labels[secondary_index] = Label::Marked;
                        queue.push_back(secondary_index);
                    }
                    Label::Outlier => {
                        queue.push_back(secondary_index);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl<'a, F, P> ClusteringAlgorithm<F, P> for DBSCAN<'a, F>
where
    F: Float,
    P: Point<F>,
{
    type Output = (Vec<Cluster<F, P>>, HashSet<usize>);

    #[must_use]
    fn fit(&self, points: &[P]) -> Self::Output {
        if points.is_empty() {
            return (Vec::new(), HashSet::new());
        }

        let neighbor_search = KDTreeSearch::new(points, self.metric);
        let mut labels = vec![Label::Undefined; points.len()];
        let mut cluster_id: usize = 0;
        for (index, point) in points.iter().enumerate() {
            if !labels[index].is_undefined() {
                continue;
            }

            let neighbors = neighbor_search.search_radius(point, self.epsilon);
            if neighbors.len() < self.min_samples {
                labels[index] = Label::Outlier;
                continue;
            }

            neighbors.iter().for_each(|neighbor| {
                labels[neighbor.index] = Label::Marked;
            });
            self.expand_cluster(
                cluster_id,
                points,
                &neighbor_search,
                &neighbors,
                &mut labels,
            );
            cluster_id += 1;
        }

        let mut cluster_map: HashMap<usize, Cluster<F, P>> = HashMap::new();
        let mut outlier_set: HashSet<usize> = HashSet::new();
        for (index, label) in labels.into_iter().enumerate() {
            match label {
                Label::Assigned(cluster_id) => {
                    let cluster = cluster_map.entry(cluster_id).or_default();
                    cluster.insert(index, &points[index]);
                }
                Label::Outlier => {
                    outlier_set.insert(index);
                }
                _ => {}
            }
        }

        let clusters: Vec<Cluster<F, P>> = cluster_map
            .into_iter()
            .filter_map(|(_, cluster)| {
                if cluster.is_empty() {
                    None
                } else {
                    Some(cluster)
                }
            })
            .collect();
        (clusters, outlier_set)
    }
}
