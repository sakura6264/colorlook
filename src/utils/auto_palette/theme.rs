use super::lab::Lab;
use super::math::number::{Float, Fraction};
use super::Swatch;

/// Trait representing a theme.
pub trait Theme {
    /// Weights a swatch based on the theme.
    ///
    /// # Arguments
    /// * `swatch` - The swatch to be weighted.
    ///
    /// # Returns
    /// The weight of the swatch.
    ///
    /// # Type Parameters
    /// * `F` - The floating type for the weight.
    #[allow(unused)]
    fn weight<F>(&self, swatch: &Swatch<F>) -> Fraction<F>
    where
        F: Float;
}

/// Struct representing a vivid theme.
pub struct Vivid;

impl Theme for Vivid {
    #[inline]
    #[allow(unused)]
    fn weight<F>(&self, swatch: &Swatch<F>) -> Fraction<F>
    where
        F: Float,
    {
        let chroma: F = swatch.color().chroma();
        let normalized = chroma.normalize(Lab::<F>::min_chroma(), Lab::<F>::max_chroma());
        Fraction::new(normalized)
    }
}

/// Struct representing a muted theme.
pub struct Muted;

impl Theme for Muted {
    #[inline]
    #[allow(unused)]
    fn weight<F>(&self, swatch: &Swatch<F>) -> Fraction<F>
    where
        F: Float,
    {
        let chroma: F = swatch.color().chroma();
        let normalized = chroma.normalize(Lab::<F>::min_chroma(), Lab::<F>::max_chroma());
        Fraction::new(F::one() - normalized)
    }
}

/// Struct representing a light theme.
pub struct Light;

impl Theme for Light {
    #[inline]
    #[allow(unused)]
    fn weight<F>(&self, swatch: &Swatch<F>) -> Fraction<F>
    where
        F: Float,
    {
        let lightness = swatch.color().lightness();
        let normalized = lightness / F::from_f64(100.0);
        Fraction::new(normalized)
    }
}

/// Struct representing a dark theme.
pub struct Dark;

impl Theme for Dark {
    #[inline]
    #[allow(unused)]
    fn weight<F>(&self, swatch: &Swatch<F>) -> Fraction<F>
    where
        F: Float,
    {
        let lightness = swatch.color().lightness();
        let normalized = lightness / F::from_f64(100.0);
        Fraction::new(F::one() - normalized)
    }
}
