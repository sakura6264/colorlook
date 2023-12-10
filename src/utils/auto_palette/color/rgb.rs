use super::super::color::xyz::XYZ;
use super::super::math::number::{Float, Number};
use super::super::white_point::WhitePoint;
use std::fmt::{Display, Formatter, Result};

/// Struct representing a color in standard RGB color space.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    /// Creates a new RGB color.
    ///
    /// # Arguments
    /// * `r` - The red component of this color.
    /// * `g` - The green component of this color.
    /// * `b` - The blue component of this color.
    ///
    /// # Returns
    /// A new RGB color.
    #[inline]
    #[allow(unused)]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Returns the min value for each component of an RGBA color.
    ///
    /// # Returns
    /// The min value for each component of an RGBA color.
    #[inline]
    #[allow(unused)]
    pub(crate) fn min_value<T: Number>() -> T {
        T::from_u8(u8::MIN)
    }

    /// Returns the max value for each component of an RGBA color.
    ///
    /// # Returns
    /// The max value for each component of an RGBA color.
    #[inline]
    #[allow(unused)]
    pub(crate) fn max_value<T: Number>() -> T {
        T::from_u8(u8::MAX)
    }

    /// Returns the red component of this color.
    ///
    /// # Returns
    /// The red component of this color.
    #[inline]
    #[allow(unused)]
    pub fn r<T: Number>(&self) -> T {
        T::from_u8(self.r)
    }

    /// Returns the green component of this color.
    ///
    /// # Returns
    /// The green component of this color.
    #[inline]
    #[allow(unused)]
    pub fn g<T: Number>(&self) -> T {
        T::from_u8(self.g)
    }

    /// Returns the blue component of this color.
    ///
    /// # Returns
    /// The blue component of this color.
    #[inline]
    #[allow(unused)]
    pub fn b<T: Number>(&self) -> T {
        T::from_u8(self.b)
    }
}

impl Display for RGB {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "RGB({r}, {g}, {b})", r = self.r, g = self.g, b = self.b,)
    }
}

impl<F, WP> From<&XYZ<F, WP>> for RGB
where
    F: Float,
    WP: WhitePoint<F>,
{
    #[inline]
    #[allow(unused)]
    fn from(xyz: &XYZ<F, WP>) -> Self {
        let f = |value: F| -> F {
            if value <= F::from_f64(0.0031308) {
                F::from_f64(12.92) * value
            } else {
                F::from_f64(1.055) * value.powf(F::from_f64(1.0 / 2.4)) - F::from_f64(0.055)
            }
        };

        let fr = f(F::from_f64(3.24097) * xyz.x
            - F::from_f64(1.537383) * xyz.y
            - F::from_f64(0.498611) * xyz.z);
        let fg = f(F::from_f64(-0.969244) * xyz.x
            + F::from_f64(1.875968) * xyz.y
            + F::from_f64(0.041555) * xyz.z);
        let fb = f(F::from_f64(0.05563) * xyz.x - F::from_f64(0.203977) * xyz.y
            + F::from_f64(1.056972) * xyz.z);

        let min_value = RGB::min_value::<F>();
        let max_value = RGB::max_value::<F>();
        let denormalize = |value: F| {
            let clamped = (value * max_value).clamp(min_value, max_value);
            clamped.round().to_u8().unwrap_or_else(RGB::min_value)
        };
        Self {
            r: denormalize(fr),
            g: denormalize(fg),
            b: denormalize(fb),
        }
    }
}