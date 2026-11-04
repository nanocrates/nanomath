use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
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

pub type Vec2f = Vec2<f32>;
pub type Vec2d = Vec2<f64>;
pub type Vec2i = Vec2<i32>;
pub type Vec2u = Vec2<u32>;

