mod drawable;

#[derive(PartialEq, Clone, Debug)]
pub enum Drawable {
    Point(drawable::point2d::Point2D),
    Line(drawable::line2d::Line2D),
}

pub use drawable::{
    line2d::Line2D as Line, point2d::Point2D as Point,
    circle::Circle, text::Text, Color, color_from_hex,
    RED, GREEN, BLUE, BLACK, WHITE,
};
