use super::point2d::Point2D;
use super::{ Draw, Color, BLACK };
use std::collections::HashMap;

/// # Text
/// a structure to represent text.
/// it takes a String reference and a point as arguments.
/// the point is the top left corner of the text's bounding box.
///
pub struct Text {
    text: String,
    pos: Point2D,
    angle: f64,
    fill: Color
}

impl Text {
    pub fn new(text: String, pos: Point2D, angle: Option<f64>, fill: Option<Color>) -> Text {
        Text {
            text, pos,
            angle: angle.unwrap_or(0.0),
            fill: fill.unwrap_or(BLACK),
        }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn pos(&self) -> Point2D {
        self.pos.clone()
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn fill(&self) -> Color {
        self.fill.clone()
    }
}

impl Draw for Text {

    /// ## Text::translate
    /// shifts the text
    fn translate(&mut self, offset: Point2D) -> &mut Self {
        self.pos += offset;
        self
    }

    /// ## Text::rotate
    /// rotates the text (currently has no effect)
    fn rotate(&mut self, angle: f64) -> &mut Self {
        self.angle += angle;
        self
    }

    /// ## Text::scale
    /// scales the text (currently has no effect)
    fn scale(&mut self, c: f64) -> &mut Self {
        let c = if c == 0.0 { 1.0 } else { c };

        self
    }
    
    /// ## Text::get_svg_tag_name
    /// always returns `"text"`
    fn get_svg_tag_name(&self) -> String {
        String::from("text")
    }
    
    /// ## Text::get_svg_tag_properties
    /// returns a `HashMap<String, String>` of the text properties
    fn get_svg_tag_properties(&self) -> HashMap<String, String> {
        let mut props = HashMap::new();
        
        // TODO: add transform property
        props.insert("x".to_string(), self.pos.x().to_string());
        props.insert("y".to_string(), self.pos.y().to_string());

        props
    }

    /// ## Text::get_svg_inner_content
    /// returns `Some(String)` containing the content
    fn get_svg_inner_content(&self) -> Option<String> {
        Some(self.text.clone())
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_translate() {
        let mut text = Text::new("test".to_string(), Point2D::new(0.0, 0.0), None, None);

        text.translate(Point2D::new(1.0, 1.0));

        assert_eq!(Point2D::new(1.0, 1.0), text.pos());
    }
}

