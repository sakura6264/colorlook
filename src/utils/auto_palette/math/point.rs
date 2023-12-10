use super::super::math::number::Float;
use num_traits::Zero;
use std::fmt::{Debug, Display, Formatter, Result};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign};

/// Trait representing a point in n-dimensional space.
///
/// # Type Parameters
/// * `F` - The type of the point's components.
pub trait Point<F: Float>:
    Copy
    + Debug
    + Index<usize, Output = F>
    + Zero
    + Add<Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> SubAssign<&'a Self>
    + Mul<F>
    + MulAssign<F>
    + Div<F>
    + DivAssign<F>
{
    /// Returns the dimension of this point.
    ///
    /// # Returns
    /// The dimension of this point.
    fn dimension(&self) -> usize;

    /// Returns the dot product of this point and the given point.
    ///
    /// # Arguments
    /// * `other` - The other point.
    ///
    /// # Returns
    /// The dot product of this point and the given point.
    fn dot(&self, other: &Self) -> F;

    /// Returns the iterator over the components of this point.
    ///
    /// # Returns
    /// The iterator over the components of this point.
    fn iter(&self) -> PointIterator<F, Self> {
        PointIterator::new(self)
    }
}

/// Struct representing an iterator over the components of a point.
///
/// # Type Parameters
/// * `F` - The type of the point's components.
/// * `P` - The type of the point.
pub struct PointIterator<'a, F: Float, P: Point<F>> {
    point: &'a P,
    index: usize,
    _marker: PhantomData<F>,
}

impl<'a, F: Float, P: Point<F>> PointIterator<'a, F, P> {
    /// Creates a new `PointIterator` instance.
    ///
    /// # Arguments
    /// * `point` - The point to iterate over.
    ///
    /// # Returns
    /// A new `PointIterator` instance.
    #[inline]
    #[must_use]
    fn new(point: &'a P) -> Self {
        Self {
            point,
            index: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, F: Float, P: Point<F>> Iterator for PointIterator<'a, F, P> {
    type Item = F;

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.point.dimension() {
            return None;
        }

        let index = self.index;
        self.index += 1;
        Some(self.point[index])
    }
}

/// Struct representing a point in 2-dimensional space.
///
/// # Type Parameters
/// * `F` - The type of the point's components.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point2<F: Float>(pub F, pub F);

impl<F> Index<usize> for Point2<F>
where
    F: Float,
{
    type Output = F;

    #[inline]
    #[must_use]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!(
                "Index {} out of bounds for dimension {}",
                index,
                self.dimension()
            ),
        }
    }
}

/// Struct representing a point in 3-dimensional space.
///
/// # Type Parameters
/// * `F` - The type of the point's components.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point3<F: Float>(pub F, pub F, pub F);

impl<F> Index<usize> for Point3<F>
where
    F: Float,
{
    type Output = F;

    #[inline]
    #[must_use]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!(
                "Index {} out of bounds for dimension {}",
                index,
                self.dimension()
            ),
        }
    }
}

/// Struct representing a point in 5-dimensional space.
///
/// # Type Parameters
/// * `F` - The type of the point's components.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point5<F: Float>(pub F, pub F, pub F, pub F, pub F);

impl<F> Index<usize> for Point5<F>
where
    F: Float,
{
    type Output = F;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            _ => panic!(
                "Index {} out of bounds for dimension {}",
                index,
                self.dimension()
            ),
        }
    }
}

macro_rules! impl_point {
  ($Point:ident { $($label:tt: $field:tt),+ }, $size:expr) => {
    impl<F> Display for $Point<F> where F: Float + Display {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}{:?}", stringify!($Point), ($(self.$field),+))
        }
    }

    impl<F> Point<F> for $Point<F> where F: Float {
        #[inline]
        #[must_use]
        fn dimension(&self) -> usize {
           $size
        }

        #[inline]
        #[must_use]
        fn dot(&self, other: &Self) -> F {
            let mut sum = F::zero();
            for i in 0..self.dimension() {
                sum += self[i] * other[i];
            }
            sum
        }
    }

    impl<F> Zero for $Point<F> where F: Float {
        #[inline]
        #[must_use]
        fn zero() -> Self {
            Self { $($field: F::zero()),+ }
        }

        #[inline]
        #[must_use]
        fn is_zero(&self) -> bool {
            $(self.$field.is_zero()) &&+
        }
    }

    impl<F> Add<$Point<F>> for $Point<F> where F: Float {
        type Output = Self;

        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            Self { $($field: self.$field + rhs.$field),+ }
        }
    }

    impl<'a, F> Add<&'a $Point<F>> for $Point<F> where F: Float {
        type Output = Self;

        #[inline]
        fn add(self, rhs: &'a Self) -> Self::Output {
            Self { $($field: self.$field + rhs.$field),+ }
        }
    }

    impl<'a, F> Sub<&'a $Point<F>> for $Point<F> where F: Float {
        type Output = Self;

        #[inline]
        fn sub(self, rhs: &'a Self) -> Self::Output {
            Self { $($field: self.$field - rhs.$field),+ }
        }
    }

    impl<F> Mul<F> for $Point<F> where F: Float {
        type Output = Self;

        #[inline]
        fn mul(self, rhs: F) -> Self::Output {
            Self { $($field: self.$field * rhs),+ }
        }
    }

    impl<F> Div<F> for $Point<F> where F: Float {
        type Output = Self;

        #[inline]
        fn div(self, divisor: F) -> Self::Output {
            if divisor.is_zero() {
                panic!("{} cannot be divided by zero", stringify!($Point));
            }
            Self { $($field: self.$field / divisor),+ }
        }
    }

    impl<'a, F> AddAssign<&'a $Point<F>> for $Point<F> where F: Float {
        #[inline]
        fn add_assign(&mut self, rhs: &'a Self) {
            $(self.$field += rhs.$field);+
        }
    }

    impl<'a, F> SubAssign<&'a $Point<F>> for $Point<F> where F: Float {
        #[inline]
        fn sub_assign(&mut self, rhs: &'a Self) {
            $(self.$field -= rhs.$field);+
        }
    }

    impl<F> MulAssign<F> for $Point<F> where F: Float {
        #[inline]
        fn mul_assign(&mut self, rhs: F) {
            $(self.$field *= rhs);+
        }
    }

    impl<F> DivAssign<F> for $Point<F> where F: Float {
        #[inline]
        fn div_assign(&mut self, divisor: F) {
            if divisor.is_zero() {
                panic!("{} cannot be divided by zero", stringify!($Point));
            }
            $(self.$field /= divisor);+
        }
    }
  }
}

impl_point!(Point2 { x: 0, y: 1 }, 2);
impl_point!(Point3 { x: 0, y: 1, z: 2 }, 3);
impl_point!(
    Point5 {
        v: 0,
        w: 1,
        x: 2,
        y: 3,
        z: 4
    },
    5
);