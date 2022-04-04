mod drawable;

#[derive(Debug)]
pub enum Drawable {
    Line(drawable::line2d::Line2D),
    Circle(drawable::circle::Circle),
    Rect2(drawable::rect2d::Rect2),
}

pub use drawable::{
    circle::Circle, line2d::Line2D as Line, vector::Vector2, text::Text,
    rect2d::Rect2, Color, BLACK, BLUE, GREEN, RED, WHITE, color_from_hex,
    Draw
};
