use super::{vector::Vector2, Color, Draw, BLUE, WHITE};
use std::collections::HashMap;

/// # Line2D
/// structure to hold lines in 2d cartesian space
/// it stores starting point, ending point, length, and angle in radians.
///
/// # Examples
/// ```
/// use std::f64::consts::{SQRT_2, FRAC_PI_4};
/// use program_core::{Line, Vector2, BLUE, WHITE};
///
/// let start = Vector2::new(0.0, 0.0);
/// let end = Vector2::new(1.0, 1.0);
/// let line = Line::new(start, end, None, None, None);
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
    start: Vector2,
    end: Vector2,
    stroke_color: Color,
    stroke_width: u8,
    fill: Color,
    len: f64,
    angle: f64,
}

impl Line2D {
    pub fn new(
        start: Vector2,
        end: Vector2,
        stroke_color: Option<Color>,
        stroke_width: Option<u8>,
        fill: Option<Color>,
    ) -> Line2D {
        Line2D {
            start,
            end,
            stroke_color: stroke_color.unwrap_or(BLUE),
            stroke_width: stroke_width.unwrap_or(5),
            fill: fill.unwrap_or(WHITE),
            len: (end - start).len(),
            angle: (end - start).arg(),
        }
    }

    pub fn start(&self) -> Vector2 {
        self.start
    }

    pub fn end(&self) -> Vector2 {
        self.end
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
    fn translate(&mut self, offset: Vector2) -> &mut Self {
        self.start += offset;
        self.end += offset;
        self
    }

    /// ## Line2D::rotate
    /// rotates the line about its starting point
    fn rotate(&mut self, angle: f64) -> &mut Self {
        self.end = self.start + self.end.rotate(angle);
        self.angle += angle;
        self
    }

    /// ## Line2D::scale
    /// scales the length of the line by moving the end point, c != 0
    fn scale(&mut self, c: f64) -> &mut Self {
        let diff = (self.end - self.start).scale(c);
        self.end = self.start + diff;
        self.len = diff.len();
        
        self
    }

    /// ## Line2D::contains
    /// checks whether the given point is on the line or not
    fn contains(&self, other: Vector2) -> bool {
        use core::f64::EPSILON;
        use std::cmp::max;

        let diff1 = other - self.start;
        let diff2 = self.end - other;
        // TODO: find a better way
        let maxlen = if diff1.len() >= diff2.len() { diff1.len() } else { diff2.len() };
        
        println!("checking for {:?}\ncross product is {}", other, diff1.cross(diff2));

        diff1.cross(diff2).abs() <= EPSILON && maxlen <= self.len() 
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
        props.insert(
            "style".to_string(),
            format!(
                "fill:{};stroke:{};stroke-width:{}",
                self.fill.to_string(),
                self.stroke_color.to_string(),
                self.stroke_width
            ),
        );

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
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

    #[test]
    fn test_translate() {
        let mut line = Line2D::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 1.0),
            None,
            None,
            None,
        );

        line.translate(Vector2::new(5.0, 5.0));

        assert_eq!(Vector2::new(5.0, 5.0), line.start());
        assert_eq!(Vector2::new(6.0, 6.0), line.end());
    }

    #[test]
    fn test_rotate() {
        let mut line = Line2D::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 1.0),
            None,
            None,
            None,
        );

        line.rotate(FRAC_PI_4);

        assert_eq!(FRAC_PI_2, line.angle());
        assert_eq!(Vector2::new(0.0, 0.0), line.start());
        assert_eq!(Vector2::new(0.0, 2f64.sqrt()), line.end());
    }

    #[test]
    fn test_scale() {
        let mut line = Line2D::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 1.0),
            None,
            None,
            None,
        );

        line.scale(3.0);
        
        use core::f64::EPSILON;
        assert!(18f64.sqrt() - line.len() <= EPSILON);
        assert_eq!(Vector2::new(0.0, 0.0), line.start());
        assert_eq!(Vector2::new(3.0, 3.0), line.end());
    }

    #[test]
    fn test_contains() {
        let line = Line2D::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(2.0, 2.0),
            None,
            None,
            None,
        );

        let v_inside1 = Vector2::new(1.0, 1.0);
        let v_inside2 = Vector2::new(2.0, 2.0);
        let v_outside1 = Vector2::new(5.0, 5.0);
        let v_outside2 = Vector2::new(17.0, 20.0);
        
        assert!(line.contains(v_inside1));
        assert!(line.contains(v_inside1));
        assert!(!line.contains(v_outside2));
        assert!(!line.contains(v_outside2));
    }
}
