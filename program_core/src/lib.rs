mod drawable;

#[derive(Debug)]
pub enum Drawable {
    Line(drawable::line2d::Line2D),
    Circle(drawable::circle::Circle),
    Rect2(drawable::rect2d::Rect2),
}

pub use drawable::canvas::props::{
    Props, CircleProps, RectProps, LineProps
};

pub use drawable::{
    circle::Circle, line2d::Line2D as Line, rect2d::Rect2,
    Color, BLACK, BLUE, GREEN, RED, WHITE, color_from_hex,
    canvas::Canvas, vector::Vector2, Draw 
};
