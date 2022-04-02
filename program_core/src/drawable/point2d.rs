use core::ops::{
    Add, AddAssign, Sub, SubAssign
};

/// # Point
/// structure to hold points in 2d cartesian space
#[derive(Clone, Debug, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign for Point2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = Point2D {
            x: self.x + rhs.x,
            y: self.x + rhs.y
        };
    }
}

impl PartialEq for Point2D {

    fn eq(&self, other: &Self) -> bool {
        use core::f64::EPSILON;
        (self.x - other.x <= EPSILON) && (self.y - other.y <= EPSILON)
    }

}

