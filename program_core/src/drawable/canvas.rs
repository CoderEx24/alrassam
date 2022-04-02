use super::{
    circle::Circle, line2d::Line2D, point2d::Point2D,
    Color, Draw, super::Drawable
};


/// # Canvas
/// a structure to hold drawables in a canvas.
/// The canvas should do all the operations of creating and manipulating drawables.
///
/// # Examples
/// ```
/// use program_core::{Canvas, Point, BLUE};
///
/// let canvas = Canvas::new(1920, 1080);
/// 
/// // add a line, the arguments have the same order as Line2D::new
/// canvas.add_line(Point::new(1.0, 1.0), Point::new(1.0, 10.0), BLUE, 1.0, None);
///
/// // export to SVG
/// canvas.export("file.svg".to_string());
///
/// ```
pub struct Canvas {
    drawables: Vec<Drawable>,
    width: u16,
    height: u16,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Canvas {
        Canvas {
            drawables: Vec::new(),
            width, height
        }
    }
    

}

