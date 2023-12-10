use super::color::lab::Lab;
use super::color::rgb::RGB;
use super::color::xyz::XYZ;
use super::color_struct::Color;
use super::image::ImageData;
use super::math::clustering::algorithm::ClusteringAlgorithm;
use super::math::clustering::cluster::Cluster;
use super::math::clustering::dbscan::algorithm::DBSCAN;
use super::math::clustering::hierarchical::algorithm::HierarchicalClustering;
use super::math::clustering::hierarchical::dendrogram::Dendrogram;
use super::math::clustering::hierarchical::linkage::CompleteLinkage;
use super::math::clustering::hierarchical::node::Node;
use super::math::distance::DistanceMetric;
use super::math::number::Float;
use super::math::point::{Point3, Point5};
use super::swatch::Swatch;
use super::{Algorithm, Theme};
use image::{ColorType, DynamicImage};
use num_traits::Zero;
use std::cmp::{Ordering, Reverse};

/// Struct representing a color palette.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
///
/// # Example
/// ```no_run
/// extern super image;
///
/// use auto_palette::{Algorithm, Palette};
///
/// let image = image::open("/path/to/image.png").unwrap();
/// let palette: Palette<f64> = Palette::extract(&image);
/// palette.swatches(5).iter().for_each(|swatch| {
///     println!("{:?}", swatch.color().to_hex_string());
///     println!("{:?}", swatch.position());
///     println!("{:?}", swatch.population());
/// });
/// ```
#[derive(Debug, PartialEq)]
pub struct Palette<F: Float> {
    swatches: Vec<Swatch<F>>,
}

impl<F> Palette<F>
where
    F: Float,
{
    /// Create a new `Palette` instance.
    ///
    /// # Arguments
    /// * `swatches` - The swatches in this palette.
    ///
    /// # Returns
    /// A new `Palette` instance.
    #[allow(unused)]
    pub fn new(swatches: Vec<Swatch<F>>) -> Self {
        Self { swatches }
    }

    /// Extract a color palette from the given image.
    ///
    /// # Arguments
    /// * `image` - The image to use for color palette extraction.
    ///
    /// # Returns
    /// A new extracted `Palette` instance.
    #[allow(unused)]
    pub fn extract(image: &DynamicImage) -> Palette<F> {
        Self::extract_with_algorithm(image, &Algorithm::DBSCAN)
    }

    /// Extract a color palette from the given image using the specified algorithm.
    ///
    /// # Arguments
    /// * `image` - The image to use for color palette extraction.
    /// * `algorithm` - The algorithm to use for color palette extraction.
    ///
    /// # Returns
    /// A new extracted `Palette` instance.
    #[allow(unused)]
    pub fn extract_with_algorithm(image: &DynamicImage, algorithm: &Algorithm) -> Palette<F> {
        let image_data = match image.color() {
            ColorType::Rgb8 => ImageData::from(&image.to_rgb8()),
            ColorType::Rgba8 => ImageData::from(&image.to_rgba8()),
            _ => unimplemented!("Unsupported image type"),
        };
        let pixels = convert_to_pixels(&image_data);

        // Merge pixels that are close in color and position, and exclude outliers.
        let pixel_clusters = algorithm.apply(&pixels);
        let (candidates, colors): (Vec<_>, Vec<_>) = pixel_clusters
            .iter()
            .filter_map(|cluster| {
                pixel_cluster_to_swatch(cluster, image_data.width(), image_data.height())
            })
            .map(|swatch| {
                let Lab { l, a, b, .. } = swatch.color().to_lab();
                let point = Point3(l, a, b);
                (swatch, point)
            })
            .unzip();

        // Merge colors with small color differences and extract the dominant swatches.
        // According to the Digital Color Imaging Handbook, a ∆E ≤ 2.3 is perceived as identical by human perception.
        let dbscan = DBSCAN::new(1, F::from_f64(2.3), &DistanceMetric::Euclidean);
        let (swatch_clusters, _) = dbscan.fit(&colors);
        let swatches = swatch_clusters
            .iter()
            .filter_map(|cluster| color_cluster_to_swatch(cluster, &candidates))
            .collect();
        Self { swatches }
    }

    /// Returns the number of swatches in this palette.
    ///
    /// # Returns
    /// The number of swatches in this palette.
    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.swatches.len()
    }

    /// Returns `true` if this palette contains no swatches.
    ///
    /// # Returns
    /// `true` if this palette contains no swatches.
    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.swatches.is_empty()
    }

    /// Finds the dominant swatches in this palette.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to return.
    ///
    /// # Returns
    /// The `n` dominant swatches in this palette.
    #[allow(unused)]
    pub fn swatches(&self, n: usize) -> Vec<Swatch<F>> {
        if self.swatches.is_empty() {
            return Vec::new();
        }

        let mut results = self.find_swatches(n, &|swatch| F::from_usize(swatch.population()));
        results.sort_by_key(|swatch| Reverse(swatch.population()));
        results
    }

    /// Finds the dominant swatches in this palette using the specified theme.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to return.
    /// * `theme` - The theme to use for color palette extraction.
    ///
    /// # Returns
    /// The `n` dominant swatches in this palette.
    #[allow(unused)]
    pub fn swatches_with_theme(&self, n: usize, theme: &impl Theme) -> Vec<Swatch<F>> {
        if self.swatches.is_empty() {
            return Vec::new();
        }

        let mut results = self.find_swatches(n, &|swatch| theme.weight(swatch).value());
        results.sort_by(|swatch1, swatch2| {
            let weight1 = theme.weight(swatch1).value();
            let weight2 = theme.weight(swatch2).value();
            weight1
                .partial_cmp(&weight2)
                .unwrap_or(Ordering::Equal)
                .reverse()
        });
        results.into_iter().take(n).collect()
    }

    #[allow(unused)]
    fn find_swatches<SF>(&self, n: usize, score_fn: &SF) -> Vec<Swatch<F>>
    where
        SF: Fn(&Swatch<F>) -> F,
    {
        let mut linkage = CompleteLinkage::new(
            &self.swatches,
            &|swatch1: &Swatch<F>, swatch2: &Swatch<F>| swatch1.distance(swatch2),
        );
        let algorithm = HierarchicalClustering::new();
        let dendrogram: Dendrogram<F> = algorithm.fit_with_linkage(&self.swatches, &mut linkage);
        let nodes = dendrogram.nodes();
        dendrogram
            .partition(n)
            .iter()
            .map(|node| self.find_swatch(nodes, node.label, score_fn))
            .collect()
    }

    #[allow(unused)]
    fn find_swatch<SF>(&self, nodes: &[Node<F>], root: usize, score_fn: &SF) -> Swatch<F>
    where
        SF: Fn(&Swatch<F>) -> F,
    {
        let root_node = &nodes[root];
        match (root_node.node1, root_node.node2) {
            (Some(node1), Some(node2)) => {
                let swatch1 = self.find_swatch(nodes, node1, score_fn);
                let swatch2 = self.find_swatch(nodes, node2, score_fn);

                let score1 = score_fn(&swatch1);
                let score2 = score_fn(&swatch2);
                let fraction = score2 / (score1 + score2);

                let color = swatch1.color().mix(swatch2.color(), fraction);
                let position = if fraction <= F::from_f64(0.5) {
                    swatch1.position()
                } else {
                    swatch2.position()
                };
                let population = swatch1.population() + swatch2.population();
                Swatch::new(color, position, population)
            }
            (Some(node1), None) => self.find_swatch(nodes, node1, score_fn),
            (None, Some(node2)) => self.find_swatch(nodes, node2, score_fn),
            (None, None) => self.swatches[root].clone(),
        }
    }
}

impl<F> Default for Palette<F>
where
    F: Float,
{
    #[allow(unused)]
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

/// Converts the given image data to pixels.
///
/// # Arguments
/// * `image_data` - The image data to convert.
///
/// # Returns
/// A vector of `Point5` instances.
#[allow(unused)]
fn convert_to_pixels<F>(image_data: &ImageData) -> Vec<Point5<F>>
where
    F: Float,
{
    let width = image_data.width() as usize;
    let width_f = F::from_u32(image_data.width());
    let height_f = F::from_u32(image_data.height());
    image_data
        .data()
        .chunks_exact(image_data.channels() as usize)
        .enumerate()
        .filter_map(|(i, chunk)| {
            let r = chunk[0];
            let g = chunk[1];
            let b = chunk[2];

            // Ignore if the alpha channel exists and the transparency value is 0.
            if chunk.len() >= 4 && chunk[3].is_zero() {
                return None;
            }

            let rgb = RGB::new(r, g, b);
            let xyz: XYZ<F> = XYZ::from(&rgb);
            let lab: Lab<F> = Lab::from(&xyz);

            let x = i % width;
            let y = i / width;

            let pixel = Point5(
                lab.l.normalize(Lab::<F>::min_l(), Lab::<F>::max_l()),
                lab.a.normalize(Lab::<F>::min_a(), Lab::<F>::max_a()),
                lab.b.normalize(Lab::<F>::min_b(), Lab::<F>::max_b()),
                F::from_usize(x) / width_f,
                F::from_usize(y) / height_f,
            );
            Some(pixel)
        })
        .collect()
}

/// Converts the given pixel cluster to a swatch.
///
/// # Arguments
/// * `pixel_cluster` - The pixel cluster to convert.
/// * `width` - The width of the source image.
/// * `height` - The height of the source image.
///
/// # Returns
/// A swatch representing the given cluster.
#[allow(unused)]
fn pixel_cluster_to_swatch<F>(
    pixel_cluster: &Cluster<F, Point5<F>>,
    width: u32,
    height: u32,
) -> Option<Swatch<F>>
where
    F: Float,
{
    let width_f = F::from_u32(width);
    let height_f = F::from_u32(height);
    if pixel_cluster.is_empty() {
        return None;
    }

    let centroid = pixel_cluster.centroid();
    let lab = Lab::<F>::new(
        centroid[0].denormalize(Lab::<F>::min_l(), Lab::<F>::max_l()),
        centroid[1].denormalize(Lab::<F>::min_a(), Lab::<F>::max_a()),
        centroid[2].denormalize(Lab::<F>::min_b(), Lab::<F>::max_b()),
    );
    let color = Color::from(&lab);

    let x = centroid[3].denormalize(F::zero(), width_f);
    let y = centroid[4].denormalize(F::zero(), height_f);
    let position = (
        x.to_u32().expect("Could not convert x to u32"),
        y.to_u32().expect("Could not convert y to u32"),
    );
    Some(Swatch::new(color, position, pixel_cluster.size()))
}

/// Converts the given color cluster to a swatch.
///
/// # Arguments
/// * `color_cluster` - The color cluster to convert.
/// * `candidates` - The candidate swatches.
///
/// # Returns
/// A swatch representing the given cluster.
#[inline]
#[allow(unused)]
fn color_cluster_to_swatch<F>(
    color_cluster: &Cluster<F, Point3<F>>,
    candidates: &[Swatch<F>],
) -> Option<Swatch<F>>
where
    F: Float,
{
    if color_cluster.is_empty() {
        return None;
    }

    let membership = color_cluster.membership();
    let Some(first_swatch) = membership.first().map(|label| candidates[*label].clone()) else {
        return None;
    };

    let best_swatch = membership
        .iter()
        .skip(1)
        .map(|label| &candidates[*label])
        .fold(first_swatch, |previous, current| {
            let population = previous.population() + current.population();
            let fraction = F::from_usize(current.population()) / F::from_usize(population);
            let color = previous.color().mix(current.color(), fraction);
            let position = if fraction <= F::from_f64(0.5) {
                previous.position()
            } else {
                current.position()
            };
            Swatch::new(color, position, population)
        });
    Some(best_swatch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    #[test]
    fn test_palette() {
        let swatches: Vec<Swatch<f64>> = vec![
            Swatch::new(Color::from(&RGB::new(0, 255, 0)), (0, 0), 1),
            Swatch::new(Color::from(&RGB::new(255, 0, 0)), (1, 1), 1),
        ];
        let palette = Palette::new(swatches);
        assert!(!palette.is_empty());
        assert_eq!(palette.len(), 2);
    }

    #[test]
    fn test_extract() {
        let data = vec![
            255, 0, 0, 255, // red
            0, 255, 0, 255, // green
            0, 0, 255, 255, // blue
            255, 255, 255, 255, // white
        ];
        let image = DynamicImage::from(RgbaImage::from_raw(2, 2, data).unwrap());
        let palette: Palette<f64> = Palette::extract(&image);

        assert!(palette.is_empty());
        assert_eq!(palette.len(), 0);
    }

    #[test]
    fn test_default() {
        let palette: Palette<f64> = Palette::default();
        assert!(palette.is_empty());
        assert_eq!(palette.len(), 0);
    }
}
