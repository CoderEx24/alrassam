mod drawable;

#[derive(Debug)]
pub enum Drawable {
    Line(drawable::line2d::Line2D),
    Circle(drawable::circle::Circle),
    Rect2(drawable::rect2d::Rect2),
}

pub use drawable::canvas::props::{CircleProps, LineProps, Props, RectProps};

pub use drawable::{
    canvas::Canvas, circle::Circle, color_from_hex, line2d::Line2D as Line, rect2d::Rect2,
    vector::Vector2, Color, Draw, BLACK, BLUE, GREEN, RED, WHITE,
};
