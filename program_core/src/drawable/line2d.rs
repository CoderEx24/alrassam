use super::point2d::Point2D;
use super::Draw;
use std::collections::HashMap;

/// # Line2D
/// structure to hold lines in 2d cartesian space
/// it stores starting point, ending point, length, and angle in radians.
///
/// # Examples
/// ```
/// use std::f64::consts::{SQRT_2, FRAC_PI_4};
/// use program_core::{Line, Point};
/// 
/// let start = Point::new(0.0, 0.0);
/// let end = Point::new(1.0, 1.0);
/// let line = Line::new(&start, &end);
///
/// assert_eq!(start, line.start());
/// assert_eq!(end, line.end());
/// assert_eq!(SQRT_2, line.len());
/// assert_eq!(FRAC_PI_4, line.angle());
///
/// ```
#[derive(PartialEq, Clone, Debug)]
pub struct Line2D {
    start: Point2D,
    end: Point2D,
    len: f64,
    angle: f64,
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

impl Draw for Line2D {
    fn get_svg_tag_name() -> String {
        String::from("line")
    }

    fn get_svg_tag_properties(&self) -> HashMap<String, String> {
        let mut props = HashMap::new();

        props.insert("x1".to_string(), self.start.x().to_string());
        props.insert("y1".to_string(), self.start.y().to_string());
        props.insert("x2".to_string(), self.end.x().to_string());
        props.insert("y2".to_string(), self.end.y().to_string());

        props
    }
}
