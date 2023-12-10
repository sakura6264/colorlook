use super::super::color::white_point::WhitePoint;
use super::super::color::xyz::XYZ;
use super::super::math::number::Float;
use super::super::white_point::D65;
use std::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;

/// Struct representing a color in CIE L*a*b* color space.
///
/// # Type Parameters
/// * `F` - The floating point type.
/// * `WP` - The white point.
///
/// # References
/// * [CIELAB color space - Wikipedia](https://en.wikipedia.org/wiki/CIELAB_color_space)
#[derive(Debug, Clone, PartialEq)]
pub struct Lab<F: Float, WP: WhitePoint<F> = D65> {
    pub l: F,
    pub a: F,
    pub b: F,
    _marker: PhantomData<WP>,
}

impl<F, WP> Lab<F, WP>
where
    F: Float,
    WP: WhitePoint<F>,
{
    /// Creates a new CIE L*a*b* color.
    ///
    /// # Arguments
    /// * `l` - The value of l.
    /// * `a` - The value of a.
    /// * `b` - The value of b.
    ///
    /// # Returns
    /// A new CIE L*a*b* color.
    #[inline]
    #[allow(unused)]
    pub fn new(l: F, a: F, b: F) -> Self {
        Self {
            l: Self::clamp_l(l),
            a: Self::clamp_a(a),
            b: Self::clamp_b(b),
            _marker: PhantomData,
        }
    }

    /// Returns the chroma of this color.
    ///
    /// # Returns
    /// The chroma of this color.
    #[inline]
    #[allow(unused)]
    pub fn chroma(&self) -> F {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }

    /// Returns the min value of l.
    ///
    /// # Returns
    /// The min value of l.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[allow(unused)]
    pub fn min_l<T: Float>() -> T {
        T::from_f64(0.0)
    }

    /// Returns the max value of l.
    ///
    /// # Returns
    /// The max value of l.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[allow(unused)]
    pub fn max_l<T: Float>() -> T {
        T::from_f64(100.0)
    }

    /// Returns the min value of a.
    ///
    /// # Returns
    /// The min value of a.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[allow(unused)]
    pub fn min_a<T: Float>() -> T {
        T::from_f64(-128.0)
    }

    /// Returns the max value of a.
    ///
    /// # Returns
    /// The max value of a.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[allow(unused)]
    pub fn max_a<T: Float>() -> T {
        T::from_f64(127.0)
    }

    /// Returns max value of b.
    ///
    /// # Returns
    /// The max value of b.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[allow(unused)]
    pub fn min_b<T: Float>() -> T {
        T::from_f64(-128.0)
    }

    /// Returns the max value of b.
    ///
    /// # Returns
    /// The max value of b.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[allow(unused)]
    pub fn max_b<T: Float>() -> T {
        T::from_f64(127.0)
    }

    /// Returns the min value of chroma.
    ///
    /// # Returns
    /// The min value of chroma.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[allow(unused)]
    pub fn min_chroma<T: Float>() -> T {
        // sqrt(0^2 + 0^2) = 0
        T::from_f64(0.0)
    }

    /// Returns the max value of chroma.
    ///
    /// # Returns
    /// The max value of chroma.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[allow(unused)]
    pub fn max_chroma<T: Float>() -> T {
        // sqrt(127^2 + 127^2) = 179.605
        T::from_f64(128.0)
    }

    #[inline]
    #[allow(unused)]
    fn clamp_l(value: F) -> F {
        value.clamp(Self::min_l(), Self::max_l())
    }

    #[inline]
    #[allow(unused)]
    fn clamp_a(value: F) -> F {
        value.clamp(Self::min_a(), Self::max_a())
    }

    #[inline]
    #[allow(unused)]
    fn clamp_b(value: F) -> F {
        value.clamp(Self::min_b(), Self::max_b())
    }
}

impl<F, WP> Display for Lab<F, WP>
where
    F: Float + Display,
    WP: WhitePoint<F>,
{
    #[inline]
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

impl<F, W> From<&XYZ<F, W>> for Lab<F, W>
where
    F: Float,
    W: WhitePoint<F>,
{
    #[inline]
    fn from(xyz: &XYZ<F, W>) -> Self {
        let epsilon = F::from_f64(6.0 / 29.0).powi(3);
        let kappa = F::from_f64(841.0 / 108.0); // ((29.0 / 6.0) ^ 2) / 3.0
        let delta = F::from_f64(4.0 / 29.0);
        let f = |t: F| -> F {
            if t > (epsilon) {
                t.cbrt()
            } else {
                kappa * t + delta
            }
        };

        let fx = f(xyz.x / W::x());
        let fy = f(xyz.y / W::y());
        let fz = f(xyz.z / W::z());

        let l = F::from_f64(116.0) * fy - F::from_f64(16.0);
        let a = F::from_f64(500.0) * (fx - fy);
        let b = F::from_f64(200.0) * (fy - fz);
        Lab::new(l, a, b)
    }
}