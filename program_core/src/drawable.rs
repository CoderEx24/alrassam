//! #drawable
//! this module will contain code for drawable objects
//! like lines, circles, rectangles, etc.
//!

pub trait Drawable {}

/// #Point
/// structure to hold points in 2d cartesian space
/// can be constructed with tuples
#[derive(PartialEq, Clone, Debug)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// #Line2D
/// structure to hold lines in 2d cartesian space
/// it stores starting point, ending point, length, and angle in radians.
///
/// #Examples
/// ```
/// use std::f64::consts::{SQRT_2, FRAC_PI_4};
/// use program_core::drawable::{Line2D, Point2D};
/// 
/// let start = Point2D::new(0.0, 0.0);
/// let end = Point2D::new(1.0, 1.0);
/// let line = Line2D::new(&start, &end);
///
/// assert_eq!(start, line.start());
/// assert_eq!(end, line.end());
/// assert_eq!(SQRT_2, line.len());
/// assert_eq!(FRAC_PI_4, line.angle());
///
/// ```
#[derive(PartialEq, Debug)]
pub struct Line2D {
    start: Point2D,
    end: Point2D,
    len: f64,
    angle: f64,
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

impl Line2D {
    pub fn new(start: &Point2D, end: &Point2D) -> Line2D {
        Line2D {
            start: start.clone(),
            end: end.clone(),
            len: ((start.x - end.x).powi(2) + (start.y - end.y).powi(2)).sqrt(),
            angle: ((start.y - end.y) / (start.x - end.x)).atan(),
        }
    }

    pub fn start(&self) -> Point2D {
        self.start.clone()
    }
    pub fn end(&self) -> Point2D {
        self.end.clone()
    }
    pub fn len(&self) -> f64 {
        self.len
    }
    pub fn angle(&self) -> f64 {
        self.angle
    }
}

impl Drawable for Line2D {}
