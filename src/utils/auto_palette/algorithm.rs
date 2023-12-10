use super::math::clustering::algorithm::ClusteringAlgorithm;
use super::math::clustering::cluster::Cluster;
use super::math::clustering::dbscan::algorithm::DBSCAN;
use super::math::clustering::gmeans::algorithm::Gmeans;
use super::math::distance::DistanceMetric;
use super::math::number::Float;
use super::math::point::Point;

/// Enum representing the supported palette extraction algorithms.
///
/// # Examples
/// ```ignore
/// use auto_palette::{Algorithm, Palette};
///
/// let image = image::open("./path/to/image.png").unwrap();
/// let palette = Palette::extract_with_algorithm(&image, &Algorithm::GMeans);
/// let palette = Palette::extract_with_algorithm(&image, &Algorithm::DBSCAN);
/// ```
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Algorithm {
    /// G-means clustering algorithm.
    GMeans,
    /// DBSCAN clustering algorithm.
    DBSCAN,
}

impl Algorithm {
    /// Applies the clustering algorithm to the given points.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    ///
    /// # Returns
    /// The clusters found by the algorithm.
    ///
    /// # Type Parameters
    /// * `F` - The float type used for calculations.
    /// * `P` - The point type used for calculations.
    pub(crate) fn apply<F, P>(&self, points: &[P]) -> Vec<Cluster<F, P>>
    where
        F: Float,
        P: Point<F>,
    {
        match self {
            Algorithm::GMeans => cluster_with_gmeans(points),
            Algorithm::DBSCAN => cluster_with_dbscan(points),
        }
    }
}

#[allow(unused)]
fn cluster_with_gmeans<F, P>(points: &[P]) -> Vec<Cluster<F, P>>
where
    F: Float,
    P: Point<F>,
{
    let gmeans = Gmeans::new(
        32, // 2^5
        8,
        16, // 4x4 grid
        F::from_f64(1e-3),
        &DistanceMetric::SquaredEuclidean,
    );
    gmeans.fit(points)
}

#[allow(unused)]
fn cluster_with_dbscan<F, P>(points: &[P]) -> Vec<Cluster<F, P>>
where
    F: Float,
    P: Point<F>,
{
    let dbscan = DBSCAN::new(
        16,                  // 4x4 grid
        F::from_f64(0.0016), // 0.04^2
        &DistanceMetric::SquaredEuclidean,
    );
    let (clusters, _) = dbscan.fit(points);
    clusters
}