use super::color_struct::Color;
use super::delta_e::DeltaE;
use super::math::number::Float;

/// Struct representing a swatch that contains a color and its position.
///
/// # Type Parameters
/// * `F` - The floating point type.
///
/// # Examples
/// ```
/// use auto_palette::color_struct::Color;
/// use auto_palette::Swatch;
/// use auto_palette::rgb::RGB;
///
/// let color = Color::<f64>::from(&RGB::new(255, 0, 64));
/// let swatch = Swatch::new(color, (90, 120), 384);
/// assert_eq!(swatch.color(), &Color::from(&RGB::new(255, 0, 64)));
/// assert_eq!(swatch.position(), (90, 120));
/// assert_eq!(swatch.population(), 384);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Swatch<F: Float> {
    color: Color<F>,
    position: (u32, u32),
    population: usize,
}

impl<F> Swatch<F>
where
    F: Float,
{
    /// Creates a new `Swatch` instance.
    ///
    /// # Arguments
    /// * `color` - The color of the swatch.
    /// * `position` - The (x, y) position of the swatch.
    /// * `population` - The population of the swatch.
    ///
    /// # Returns
    /// A `Swatch` instance.
    #[allow(unused)]
    pub fn new(color: Color<F>, position: (u32, u32), population: usize) -> Self {
        Self {
            color,
            position,
            population,
        }
    }

    /// Returns the color of this swatch.
    ///
    /// # Returns
    /// A reference of color of this swatch.
    #[allow(unused)]
    pub fn color(&self) -> &Color<F> {
        &self.color
    }

    /// Returns the (x, y) position of this swatch.
    ///
    /// # Returns
    /// The (x, y) position of this swatch.
    #[allow(unused)]
    pub fn position(&self) -> (u32, u32) {
        self.position
    }

    /// Returns the population of this swatch.
    ///
    /// # Returns
    /// The population of this swatch.
    #[allow(unused)]
    pub fn population(&self) -> usize {
        self.population
    }

    /// Calculates the distance between this swatch and another swatch.
    ///
    /// # Arguments
    /// * `other` - The other swatch.
    ///
    /// # Returns
    /// The distance between this swatch and another swatch.
    ///
    /// # Type Parameters
    /// * `F` - The floating type for the distance.
    #[inline]
    #[allow(unused)]
    pub(super) fn distance(&self, other: &Self) -> F {
        self.color.difference(&other.color, &DeltaE::CIE2000)
    }
}