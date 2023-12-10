use super::super::super::super::math::clustering::algorithm::ClusteringAlgorithm;
use super::super::super::super::math::clustering::cluster::Cluster;
use super::super::super::super::math::clustering::cmp::Priority;
use super::super::super::super::math::distance::DistanceMetric;
use super::super::super::super::math::neighbors::kdtree::search::KDTreeSearch;
use super::super::super::super::math::neighbors::search::NeighborSearch;
use super::super::super::super::math::number::Float;
use super::super::super::super::math::point::Point;
use super::super::super::super::math::stats::{anderson_darling_test, standardize};
use std::collections::BinaryHeap;

/// Struct representing G-means clustering algorithm.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
///
/// # References
/// * [The Gaussian-means (G-means) algorithm](https://proceedings.neurips.cc/paper_files/paper/2003/file/234833147b97bb6aed53a8f4f1c7a7d8-Paper.pdf)
#[derive(Debug, PartialEq)]
pub struct Gmeans<'a, F>
where
    F: Float,
{
    max_k: usize,
    max_iter: usize,
    min_cluster_size: usize,
    tolerance: F,
    metric: &'a DistanceMetric,
}

impl<'a, F> Gmeans<'a, F>
where
    F: Float,
{
    /// Creates a new `Gmeans` instance.
    ///
    /// # Arguments
    /// * `max_k` - The maximum number of clusters.
    /// * `max_iter` - The maximum number of iterations.
    /// * `min_cluster_size` - The minimum number of points required to form a cluster.
    /// * `tolerance` - The minimum change in cluster centroids required to continue iterating.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `Gmeans` instance.
    #[must_use]
    pub fn new(
        max_k: usize,
        max_iter: usize,
        min_cluster_size: usize,
        tolerance: F,
        metric: &'a DistanceMetric,
    ) -> Self {
        assert!(
            max_k >= 2,
            "The maximum number of clusters must be at least 2."
        );
        Self {
            max_k,
            max_iter,
            min_cluster_size,
            tolerance,
            metric,
        }
    }

    #[must_use]
    fn split<P: Point<F>>(
        &self,
        cluster: &Cluster<F, P>,
        points: &[P],
    ) -> (Cluster<F, P>, Cluster<F, P>) {
        let membership = cluster.membership();
        let mut clusters = Vec::with_capacity(2);
        for i in 0..2 {
            let index = cluster.size() * (i + 1) / 3;
            let centroid_index = membership[index];
            let centroid = points[centroid_index];
            clusters.push(Cluster::new(centroid));
        }

        for _ in 0..self.max_iter {
            let converged = self.assign(&mut clusters, membership, points);
            if converged {
                break;
            }
        }
        (clusters[0].clone(), clusters[1].clone())
    }

    #[must_use]
    fn assign<P: Point<F>>(
        &self,
        clusters: &mut [Cluster<F, P>],
        indices: &[usize],
        points: &[P],
    ) -> bool {
        let mut centroids = Vec::with_capacity(clusters.len());
        for cluster in clusters.iter_mut() {
            centroids.push(*cluster.centroid());
            cluster.clear();
        }

        let neighbor_search = KDTreeSearch::new(&centroids, self.metric);
        for &index in indices.iter() {
            let point = &points[index];
            let Some(nearest) = neighbor_search.search_nearest(point) else {
                continue;
            };
            clusters[nearest.index].insert(index, point);
        }

        let mut converged = true;
        for (cluster, old_centroid) in clusters.iter_mut().zip(centroids) {
            if cluster.is_empty() {
                continue;
            }

            let difference = self.metric.measure(&old_centroid, cluster.centroid());
            if difference >= self.tolerance {
                converged = false;
            }
        }
        converged
    }
}

impl<'a, F, P> ClusteringAlgorithm<F, P> for Gmeans<'a, F>
where
    F: Float,
    P: Point<F>,
{
    type Output = Vec<Cluster<F, P>>;

    #[must_use]
    fn fit(&self, points: &[P]) -> Self::Output {
        if points.is_empty() {
            return Vec::new();
        }

        let cluster = {
            let median = points.len() / 2;
            Cluster::new(points[median])
        };

        let mut clusters = vec![cluster];
        let membership: Vec<usize> = (0..points.len()).collect();
        if self.assign(&mut clusters, &membership, points) {
            return clusters;
        }

        let mut heap = BinaryHeap::from_iter(clusters.into_iter().map(|cluster| {
            let priority = cluster.size();
            Priority::new(cluster, priority)
        }));
        let mut clusters = Vec::with_capacity(self.max_k);
        while clusters.len() < self.max_k {
            let Some(largest) = heap.pop() else {
                break;
            };

            let largest_size = largest.1;
            if largest_size < self.min_cluster_size || largest_size <= 1 {
                break;
            }

            let largest_cluster = largest.0;
            let (cluster1, cluster2) = self.split(&largest_cluster, points);
            let centroid1 = cluster1.centroid();
            let centroid2 = cluster2.centroid();

            // Anderson Darling test
            let v = centroid1.sub(centroid2);
            let vp = v.dot(&v);
            let mut x = Vec::with_capacity(largest_size);
            for &index in largest_cluster.membership().iter() {
                let point = &points[index];
                x.push(point.dot(&v) / vp);
            }
            standardize(&mut x);
            let Some(score) = anderson_darling_test(&x) else {
                break;
            };
            if score < F::from_f64(1.8692) {
                clusters.push(cluster1);
                clusters.push(cluster2);
            } else {
                let priority1 = cluster1.size();
                heap.push(Priority::new(cluster1, priority1));

                let priority2 = cluster2.size();
                heap.push(Priority::new(cluster2, priority2));
            }
        }
        clusters
    }
}