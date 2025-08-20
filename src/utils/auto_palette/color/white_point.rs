use super::super::math::number::Float;

/// Trait representing a white point.
///
/// # Type Parameters
/// * `F` - The floating point type.
///
/// # References
/// * [White point - Wikipedia](https://en.wikipedia.org/wiki/White_point)
pub trait WhitePoint<F>: Clone + Default + PartialEq
where
    F: Float,
{
    /// Returns the value of x.
    ///
    /// # Returns
    /// The value of x.
    #[allow(unused)]
    fn x() -> F;

    /// Returns the value of y.
    ///
    /// # Returns
    /// The value of y.
    #[allow(unused)]
    fn y() -> F;

    /// Returns the value of z.
    ///
    /// # Returns
    /// The value of z.
    #[allow(unused)]
    fn z() -> F;
}

/// Struct representing CIE standard illuminant D65
///
/// # References
/// * [Illuminant D65](https://en.wikipedia.org/wiki/Illuminant_D65)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct D65;

impl<F> WhitePoint<F> for D65
where
    F: Float,
{
    #[inline]
    #[allow(unused)]
    fn x() -> F {
        F::from_f64(0.95046)
    }

    #[inline]
    #[allow(unused)]
    fn y() -> F {
        F::from_f64(1.0)
    }

    #[inline]
    #[allow(unused)]
    fn z() -> F {
        F::from_f64(1.08906)
    }
}
