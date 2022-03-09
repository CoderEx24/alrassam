use super::point2d::Point2D;
use super::Draw;
use std::collections::HashMap;

/// # Text
/// a structure to represent text.
/// it takes a String reference and a point as arguments.
/// the point is the top left corner of the text's bounding box.
///
pub struct Text {
    text: String,
    pos: Point2D,
}

impl Text {
    pub fn new(text: String, pos: Point2D) -> Text {
        Text {
            text, pos
        }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn pos(&self) -> Point2D {
        self.pos.clone()
    }
}

impl Draw for Text {
    fn get_svg_tag_name() -> String {
        String::from("text")
    }

    fn get_svg_tag_properties(&self) -> HashMap<String, String> {
        let mut props = HashMap::new();
        
        props.insert("x".to_string(), self.pos.x().to_string());
        props.insert("y".to_string(), self.pos.y().to_string());

        props
    }
}

