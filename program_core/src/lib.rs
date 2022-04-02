mod drawable;

#[derive(PartialEq, Clone, Debug)]
pub enum Drawable {
    Point(drawable::point2d::Point2D),
    Line(drawable::line2d::Line2D),
}

pub use drawable::{
    circle::Circle, color_from_hex, line2d::Line2D as Line, point2d::Point2D as Point, text::Text,
    Color, BLACK, BLUE, GREEN, RED, WHITE,
};
