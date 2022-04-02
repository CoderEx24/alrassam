use super::{Draw, Color, BLUE, WHITE, point2d::Point2D};
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
/// let line = Line::new(&start, &end, None, None, None);
///
/// assert_eq!(start, line.start());
/// assert_eq!(end, line.end());
/// assert_eq!(BLUE, line.stroke_color());
/// assert_eq!(5, line.stroke_width());
/// assert_eq!(WHITE, line.fill());
/// assert_eq!(SQRT_2, line.len());
/// assert_eq!(FRAC_PI_4, line.angle());
///
/// ```
#[derive(PartialEq, Clone, Debug)]
pub struct Line2D {
    start: Point2D,
    end: Point2D,
    stroke_color: Color,
    stroke_width: u8,
    fill: Color,
    len: f64,
    angle: f64,
}

impl Line2D {
    pub fn new(start: &Point2D, end: &Point2D, stroke_color: Option<Color>, stroke_width: Option<u8>, fill: Option<Color>) -> Line2D {
        Line2D {
            start: start.clone(),
            end: end.clone(),
            stroke_color: stroke_color.unwrap_or(BLUE),
            stroke_width: stroke_width.unwrap_or(5),
            fill: fill.unwrap_or(WHITE),
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

    pub fn stroke_color(&self) -> Color {
        self.stroke_color.clone()
    }

    pub fn stroke_width(&self) -> u8 {
        self.stroke_width
    }

    pub fn fill(&self) -> Color {
        self.fill.clone()
    }

    pub fn len(&self) -> f64 {
        self.len
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }
}

impl Draw for Line2D {

    /// ## Line2D::translate
    /// shifts the starting and ending points
    fn translate(&mut self, offset: Point2D) -> &mut Self {
        self.start += offset;
        self.end += offset;
        self
    }

    /// ## Line2D::rotate
    /// rotates the line about its starting point
    fn rotate(&mut self, angle: f64) -> &mut Self {
        self.end = self.start + Point2D { x: self.len() * angle.cos(), y: self.len() * angle.sin() };
        self
    }

    /// ## Line2D::scale
    /// scales the length of the line by moving the end point, c != 0
    fn scale(&mut self, c: f64) -> &mut Self {
        // make sure we actually have a valid value
        let c = if c == 0.0 { 1.0 } else { c };
        
        self.end = self.start + Point2D { x: c * self.end.x, y: c * self.end.y };

        self
    }
    
    /// ## Line2D::get_svg_tag_name
    /// always returns `"line"`
    fn get_svg_tag_name(&self) -> String {
        String::from("line")
    }

    /// ## Line2D::get_svg_tag_properties
    /// returns a `HashMap<String, String>` of the properties of the line tag
    fn get_svg_tag_properties(&self) -> HashMap<String, String> {
        let mut props = HashMap::new();

        props.insert("x1".to_string(), self.start.x().to_string());
        props.insert("y1".to_string(), self.start.y().to_string());
        props.insert("x2".to_string(), self.end.x().to_string());
        props.insert("y2".to_string(), self.end.y().to_string());
        props.insert("style".to_string(), 
                     format!("fill:{};stroke:{};stroke-width:{}", self.fill.to_string(), 
                             self.stroke_color.to_string(), self.stroke_width));

        props
    }

    /// ## Line2D::get_svg_inner_content
    /// always returns `None`
    fn get_svg_inner_content(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use std::f64::consts::{FRAC_PI_4, FRAC_PI_2};

    #[test]
    fn test_translate() {
        let mut line = Line2D::new(&Point2D::new(0.0, 0.0), &Point2D::new(1.0, 1.0), None, None, None);
        
        line.translate(Point2D::new(5.0, 5.0));

        assert_eq!(Point2D::new(5.0, 5.0), line.start());
        assert_eq!(Point2D::new(6.0, 6.0), line.end());
    }

    #[test]
    fn test_rotate() {
        let mut line = Line2D::new(&Point2D::new(0.0, 0.0), &Point2D::new(1.0, 1.0), None, None, None);

        line.rotate(FRAC_PI_4);

        assert_eq!(FRAC_PI_2, line.angle());
        assert_eq!(Point2D::new(0.0, 0.0), line.start());
        assert_eq!(Point2D::new(0.0, 1.0), line.end());
    }

    #[test]
    fn test_scale() {
        let mut line = Line2D::new(&Point2D::new(0.0, 0.0), &Point2D::new(1.0, 1.0), None, None, None);

        line.scale(2.0);

        assert_eq!(8f64.sqrt(), line.len());
        assert_eq!(Point2D::new(0.0, 0.0), line.start());
        assert_eq!(Point2D::new(2.0, 2.0), line.end());
    }

}

