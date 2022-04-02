use super::point2d::Point2D;
use super::Draw;
use std::collections::HashMap;
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
            area: PI * radius.powi(2),
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

impl Draw for Circle {
    /// ## Circle::translate
    /// shifts the center of the circle
    fn translate(&mut self, offset: Point2D) -> &mut Self {
        self.center += offset;
        self
    }

    /// ## Circle::rotate
    /// does nothing
    // rotating a circle is meaningless
    fn rotate(&mut self, _: f64) -> &mut Self {
        self
    }

    /// ## Circle::scale
    /// scales the radius of the circle
    fn scale(&mut self, c: f64) -> &mut Self {
        // scaling by zero shouldn't happen, but just in case
        self.radius *= if c == 0.0 { 1.0 } else { c };
        self
    }

    /// ## Circle::get_svg_tag_name
    /// always returns `"circle"`
    fn get_svg_tag_name(&self) -> String {
        String::from("circle")
    }

    /// ## Circle::get_svg_tag_properties
    /// returns a `HashMap<String, String>`, with properties names as keys (like cx)
    /// and properties values as values
    fn get_svg_tag_properties(&self) -> HashMap<String, String> {
        let mut props = HashMap::new();

        props.insert("cx".to_string(), self.center.x().to_string());
        props.insert("cy".to_string(), self.center.y().to_string());
        props.insert("r".to_string(), self.radius.to_string());

        props
    }

    /// ## Circle::get_svg_inner_content
    /// always returns `None`
    fn get_svg_inner_content(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_translate() {
        let mut circle = Circle::new(&Point2D::new(0.0, 0.0), 5.0);

        circle.translate(Point2D::new(1.0, 1.0));

        assert_eq!(Point2D::new(1.0, 1.0), circle.center());
        assert_eq!(5.0, circle.radius());
    }

    #[test]
    fn test_scale() {
        let mut circle = Circle::new(&Point2D::new(0.0, 0.0), 5.0);

        circle.scale(2.0);

        assert_eq!(Point2D::new(0.0, 0.0), circle.center());
        assert_eq!(10.0, circle.radius());
    }
}
