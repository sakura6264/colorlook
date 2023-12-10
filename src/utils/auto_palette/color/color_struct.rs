use super::super::delta_e::DeltaE;
use super::super::lab::Lab;
use super::super::math::number::Float;
use super::super::rgb::RGB;
use super::super::white_point::{WhitePoint, D65};
use super::super::xyz::XYZ;
use std::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;

/// Struct representing a color.
///
/// # Type Parameters
/// * `F` - The floating point type.
/// * `WP` - The white point.
///
/// # Examples
/// ```
/// use statrs::assert_almost_eq;
/// use auto_palette::color_struct::Color;
/// use auto_palette::rgb::RGB;
///
/// let yellow = RGB::new(255, 255, 0);
/// let color = Color::<f64>::from(&yellow);
/// assert_eq!(color.is_light(), true);
/// assert_eq!(color.is_dark(), false);
/// assert_almost_eq!(color.lightness(), 97.1385, 1e-4);
/// assert_almost_eq!(color.chroma(), 96.9126, 1e-4);
/// assert_almost_eq!(color.hue(), 102.8544, 1e-4);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Color<F: Float, WP = D65> {
    l: F,
    a: F,
    b: F,
    _marker: PhantomData<WP>,
}

impl<F, WP> Color<F, WP>
where
    F: Float,
    WP: WhitePoint<F>,
{
    /// Creates a new `Color` instance.
    ///
    /// # Arguments
    /// * `l` - The value of l.
    /// * `a` - The value of a.
    /// * `b` - The value of b.
    ///
    /// # Returns
    /// A new `Color` instance.
    #[allow(unused)]
    fn new(l: F, a: F, b: F) -> Self {
        Self {
            l,
            a,
            b,
            _marker: PhantomData,
        }
    }

    /// Returns whether this color is light.
    ///
    /// # Returns
    /// `true` if this color is light, otherwise `false`.
    #[inline]
    #[allow(unused)]
    pub fn is_light(&self) -> bool {
        self.l > F::from_f64(50.0)
    }

    /// Returns whether this color is dark.
    ///
    /// # Returns
    /// `true` if this color is dark, otherwise `false`.
    #[inline]
    #[allow(unused)]
    pub fn is_dark(&self) -> bool {
        !self.is_light()
    }

    /// Returns the lightness of this color.
    /// The range of the lightness is [0, 100).
    ///
    /// # Returns
    /// The lightness of this color.
    #[inline]
    #[allow(unused)]
    pub fn lightness(&self) -> F {
        self.l
    }

    /// Returns the chroma of this color.
    /// The range of the chroma is [0, 128).
    ///
    /// # Returns
    /// The chroma of this color.
    #[inline]
    #[allow(unused)]
    pub fn chroma(&self) -> F {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }

    /// Returns the hue angle of this color.
    /// The range of the hue angle is [0, 360).
    ///
    /// # Returns
    /// The hue of this color.
    #[inline]
    #[allow(unused)]
    pub fn hue(&self) -> F {
        let hue = self.b.atan2(self.a).to_degrees();
        if hue < F::zero() {
            hue + F::from_f64(360.0)
        } else {
            hue
        }
    }

    /// Mixes this color with another color with the given ratio.
    ///
    /// # Arguments
    /// * `other` - The other color.
    /// * `ratio` - The ratio of the other color.
    ///
    /// # Returns
    /// A mixed color.
    #[inline]
    #[allow(unused)]
    pub fn mix(&self, other: &Color<F, WP>, ratio: F) -> Color<F, WP> {
        let l = self.l + (other.l - self.l) * ratio;
        let a = self.a + (other.a - self.a) * ratio;
        let b = self.b + (other.b - self.b) * ratio;
        let lab = Lab::<F, WP>::new(l, a, b);
        Self::new(lab.l, lab.a, lab.b)
    }

    /// Calculates the color difference between this color and another color.
    /// The color difference is calculated by the given delta E metric.
    ///
    /// # Arguments
    /// * `other` - The other color.
    /// * `metric` - The delta E metric.
    ///
    /// # Returns
    /// The color difference.
    #[inline]
    #[allow(unused)]
    pub fn difference(&self, other: &Color<F, WP>, metric: &DeltaE) -> F {
        let lab1 = self.to_lab();
        let lab2 = other.to_lab();
        metric.measure(&lab1, &lab2)
    }

    /// Converts this color to an RGB color.
    ///
    /// # Returns
    /// A converted RGB color.
    #[inline]
    #[allow(unused)]
    pub fn to_rgb(&self) -> RGB {
        RGB::from(&self.to_xyz())
    }

    /// Converts this color to an XYZ color.
    ///
    /// # Returns
    /// A converted XYZ color.
    #[inline]
    #[allow(unused)]
    pub fn to_xyz(&self) -> XYZ<F, WP> {
        XYZ::<F, WP>::from(&self.to_lab())
    }

    /// Converts this color to a CIE L*a*b* color.
    ///
    /// # Returns
    /// A converted CIE L*a*b* color.
    #[inline]
    #[allow(unused)]
    pub fn to_lab(&self) -> Lab<F, WP> {
        Lab::<F, WP>::new(self.l, self.a, self.b)
    }

    /// Converts this color to a hex string representation.
    ///
    /// # Returns
    /// A hex string representation.
    #[inline]
    #[allow(unused)]
    pub fn to_hex_string(&self) -> String {
        let rgb = self.to_rgb();
        format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b)
    }
}

impl<F, WP> Display for Color<F, WP>
where
    F: Float + Display,
    WP: WhitePoint<F>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Lab({l:.4}, {a:.4}, {b:.4})",
            l = self.l,
            a = self.a,
            b = self.b
        )
    }
}

impl<F, WP> From<&RGB> for Color<F, WP>
where
    F: Float,
    WP: WhitePoint<F>,
{
    #[allow(unused)]
    fn from(rgb: &RGB) -> Self {
        let xyz = XYZ::<F, WP>::from(rgb);
        let lab = Lab::<F, WP>::from(&xyz);
        Self::new(lab.l, lab.a, lab.b)
    }
}

impl<F, WP> From<&XYZ<F, WP>> for Color<F>
where
    F: Float,
    WP: WhitePoint<F>,
{
    #[allow(unused)]
    fn from(xyz: &XYZ<F, WP>) -> Self {
        let lab = Lab::<F, WP>::from(xyz);
        Self::new(lab.l, lab.a, lab.b)
    }
}

impl<F, WP> From<&Lab<F, WP>> for Color<F>
where
    F: Float,
    WP: WhitePoint<F>,
{
    #[allow(unused)]
    fn from(lab: &Lab<F, WP>) -> Self {
        Self::new(lab.l, lab.a, lab.b)
    }
}