use super::{ vector::Vector2, Color, Draw, BLACK, WHITE };
use std::collections::HashMap;

/// # rect2d::Rect2
/// a structure to represent rectangles.
///
/// # Example
/// ```
/// use program_core::{ Rect2, Draw, Vector2, BLUE, RED };
/// use core::f64::consts::{ FRAC_PI_4, SQRT_2 };
///
/// let start = Vector2::new(0.0, 0.0);
/// let end = Vector2::new(1.0, 1.0);
/// let mut rect = Rect2::new(start, end, Some(BLUE), Some(5), Some(RED));
///
/// let point_inside = Vector2::new(0.5, 0.5);
/// let point_outside = Vector2::new(2.0, 2.0);
///
/// assert!(rect.contains(point_inside), "checking for point inside the rect");
/// assert!(!rect.contains(point_outside), "checking for point outside the rect");
///
/// rect.translate(Vector2::new(1.0, 1.0))
///     .rotate(FRAC_PI_4)
///     .scale(2.0);
///
/// assert_eq!(Vector2::new(1.0, 1.0), rect.start(), "checking for the start pos after transformation");
/// // try to apply the transformation yourself to see why it is this weird value :3
/// assert_eq!(Vector2::new(1.0, 1.0 + 2.0 * SQRT_2), rect.end(), "checking for end pos after transformation");
/// assert_eq!(Vector2::new(2.0, 2.0), rect.dimensions());
/// ```
#[derive(Debug)]
pub struct Rect2 {
    start: Vector2,
    diagonal: Vector2,
    angle: f64,
    stroke_color: Color,
    stroke_width: u8,
    fill: Color,
}

impl Rect2 {
    pub fn new(start: Vector2, end: Vector2, stroke_color: Option<Color>, stroke_width: Option<u8>, fill: Option<Color>) -> Rect2 {
        Rect2 {
            start,
            diagonal: end - start,
            angle: 0.0,
            stroke_color: stroke_color.unwrap_or(BLACK),
            stroke_width: stroke_width.unwrap_or(12),
            fill: fill.unwrap_or(WHITE),
        }
    }

    pub fn new_square(start: Vector2, len: f64, stroke_color: Option<Color>, stroke_width: Option<u8>, fill: Option<Color>) -> Rect2 {
        Rect2 {
            start,
            diagonal: Vector2::new(len, -len),
            angle: 0.0,
            stroke_color: stroke_color.unwrap_or(BLACK),
            stroke_width: stroke_width.unwrap_or(12),
            fill: fill.unwrap_or(WHITE),
        }
    }

    pub fn start(&self) -> Vector2 {
        self.start
    }

    pub fn end(&self) -> Vector2 {
        self.start + self.diagonal
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn dimensions(&self) -> (f64, f64) {
        (self.diagonal.clone().rotate(-self.angle).x().abs() , self.diagonal.clone().rotate(-self.angle).y().abs())
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
}

impl Draw for Rect2 {
    /// ## Rect2::translate
    /// shifts the rectangle by shifting its top left corner
    fn translate(&mut self, offset: Vector2) -> &mut Self {
        self.start += offset;
        self
    }

    /// ## Rect2::rotate
    /// rotates the rectangle by rotating the diagonal
    fn rotate(&mut self, angle: f64) -> &mut Self {
        self.angle += angle;
        self.diagonal.rotate(angle);
        self
    }

    /// ## Rect2::scale
    /// scales the rectangle by scaling the diagonal
    fn scale(&mut self, c: f64) -> &mut Self {
        self.diagonal.scale(c);
        self
    }
    
    /// ## Rect2::contains
    /// checks whether the given point is in the rectangle or not
    fn contains(&self, point: Vector2) -> bool {
        let diff = point - self.start;

        (diff.x().abs() <= self.diagonal.x().abs()) && (diff.y().abs() <= self.diagonal.y().abs())
    }

    /// ## Rect2::get_svg_tag_name
    /// always returns `"rect"`
    fn get_svg_tag_name(&self) -> String {
        "rect".to_string()
    }
    
    /// ## Rect2::get_svg_tag_properties
    /// returns a `HashMap<String, String>` with the properties of the rect tag
    fn get_svg_tag_properties(&self) -> HashMap<String, String> {
        let mut props = HashMap::new();
        
        props.insert("x".to_string(), self.start.x().to_string());
        props.insert("y".to_string(), self.start.y().to_string());
        props.insert("width".to_string(), self.diagonal.x().to_string());
        props.insert("height".to_string(), self.diagonal.y().to_string());
        props.insert("style".to_string(), format!("fill:{};stroke:{};stroke_width:{};", self.fill.to_string(), self.stroke_color.to_string(), self.stroke_width));
        
        props
    }

    fn get_svg_inner_content(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_dimensions() {
        let rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0), None, None, None);

        assert_eq!(Vector2::new(1.0, 1.0), rect.dimensions());
    }

    #[test]
    fn test_make_square() {
        let sqr = Rect2::new_square(Vector2::new(0.0, 10.0), 10.0, None, None, None);

        assert_eq!(Vector2::new(0.0, 10.0), sqr.start());
        assert_eq!(Vector2::new(10.0, 0.0), sqr.end());
        assert_eq!(Vector2::new(10.0, 10.0), sqr.dimensions());
    }

    #[test]
    fn test_translate() {
        let mut rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0), None, None, None);

        rect.translate(Vector2::new(2.0, 2.0));

        assert_eq!(Vector2::new(2.0, 2.0), rect.start());
        assert_eq!(Vector2::new(3.0, 3.0), rect.end());
    }

    #[test]
    fn test_rotate() {
        use core::f64::consts::{ FRAC_PI_4, FRAC_PI_2 };

        let mut rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0), None, None, None);

        rect.rotate(FRAC_PI_4);
        assert_eq!(FRAC_PI_4, rect.angle());
        assert_eq!(Vector2::new(0.0, 0.0), rect.start());
        assert_eq!(Vector2::new(0.0, 2f64.sqrt()), rect.end());
    }

    #[test]
    fn test_scale() {
        let mut rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0), None, None, None);

        rect.scale(2.0);

        assert_eq!(Vector2::new(0.0, 0.0), rect.start());
        assert_eq!(Vector2::new(2.0, 2.0), rect.end());
        assert_eq!(Vector2::new(2.0, 2.0), rect.dimensions());
    }

    #[test]
    fn test_contains() {
        let rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0), None, None, None);

        let v_inside = [Vector2::new(0.5, 0.5), Vector2::new(0.75, 0.0), Vector2::new(1.0, 0.0), Vector2::new(1.0, 1.0)];
        let v_outside = [Vector2::new(2.0, 2.0), Vector2::new(1.0, 1.1), Vector2::new(0.0, 5.0), Vector2::new(12.0, 16.4)];

        assert!(rect.contains(v_inside[0]));
        assert!(rect.contains(v_inside[1]));
        assert!(rect.contains(v_inside[2]));
        assert!(rect.contains(v_inside[3]));
        
        assert!(!rect.contains(v_outside[0]));
        assert!(!rect.contains(v_outside[1]));
        assert!(!rect.contains(v_outside[2]));
        assert!(!rect.contains(v_outside[3]));
    }

}
