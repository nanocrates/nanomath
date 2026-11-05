//! Fast, stack-allocated set-size vector types for Rust.

use core::fmt;

/// A two-dimensional vector type storing two components: `x` and `y`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vec2<T> {

    /// The first component of the vector (typically representative of the `x`-coordinate).
    pub x: T,

    /// The second component of the vector (typically representative of the `y`-coordinate).
    pub y: T,
}

impl<T> Vec2<T> {
    
    /// Create a new two-dimensional vector with the given `x` and `y` coordinates.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(tuple: (T, T)) -> Self {
        Self { x: tuple.0, y: tuple.1 }
    }
}

impl<T> From<Vec2<T>> for (T, T) {
    fn from(vec: Vec2<T>) -> Self {
        (vec.x, vec.y)
    }
}

impl<T: fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A two-dimensional vector. containing 2 elements of type [`f32`] (floats).
/// Identical to `Vec2<f32>`.
pub type Vec2f = Vec2<f32>;

/// A two-dimensional vector. containing 2 elements of type [`f64`] (doubles).
/// Identical to `Vec2<f64>`.
pub type Vec2d = Vec2<f64>;

/// A two-dimensional vector. containing 2 elements of type [`i32`] (signed integer).
/// Identical to `Vec2<i32>`.
pub type Vec2i = Vec2<i32>;

/// A two-dimensional vector. containing 2 elements of type [`u32`] (unsigned integer).
/// Identical to `Vec2<u32>`.
pub type Vec2u = Vec2<u32>;

