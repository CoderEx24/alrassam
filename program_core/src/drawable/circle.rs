use super::point2d::Point2D;
use super::Draw;
use std::f64::consts::PI;

/// # Circle
/// structure to hold circles in 2d carteian space
/// it takes 2 arguments, a point as the center and a float as the radius
///
/// # Examples
/// ```
/// use program_core::{Point, Circle};
/// use std::f64::consts::PI;
///
/// let center = Point::new(1.0, 1.0);
/// let radius = 5f64;
///
/// let circle = Circle::new(&center, radius);
///
/// assert_eq!(5f64, circle.radius());
/// assert_eq!(center, circle.center());
/// assert_eq!(PI * 10f64, circle.circumference());
/// assert_eq!(PI * 25f64, circle.area());
///
/// ```
pub struct Circle {
    center: Point2D,
    radius: f64,
    circumference: f64,
    area: f64,

}

impl Circle {
    pub fn new(center: &Point2D, radius: f64) -> Circle {
        Circle {
            center: center.clone(),
            radius,
            circumference: 2f64 * PI * radius,
            area: PI * radius.powi(2)
        }
    }
    
    pub fn center(&self) -> Point2D {
        self.center.clone()
    }

    pub fn radius(&self) -> f64 {
        self.radius 
    }

    pub fn circumference(&self) -> f64 {
        self.circumference 
    }

    pub fn area(&self) -> f64 {
        self.area
    }

}

impl Draw for Circle {}
