use super::{
    super::Drawable, circle::Circle, line2d::Line2D, rect2d::Rect2, vector::Vector2, Color, Draw,
};
use std::io::Error;

/// # props
/// a module that contains proxy structures.
/// these structures will be using to communicate information about drawables
/// to the client code.
///
/// these objects will almost always be immutable, and changing their attributes will always
/// have no effect.
///
/// to modify the drawables, use the available methods on `Canvas`
pub mod props {

    use super::{Color, Vector2};

    /// # LineProps
    /// a proxy structure for Line2D
    #[derive(Debug, PartialEq, Clone)]
    pub struct LineProps {
        pub start: Vector2,
        pub end: Vector2,
        pub angle: f64,
        pub len: f64,
        pub stroke_color: Color,
        pub stroke_width: u8,
        pub fill: Color,
    }

    /// # RectProps
    /// a proxy structure for Rect2
    #[derive(Debug, PartialEq, Clone)]
    pub struct RectProps {
        pub start: Vector2,
        pub end: Vector2,
        pub angle: f64,
        pub stroke_color: Color,
        pub stroke_width: u8,
        pub fill: Color,
    }

    /// # CircleProps
    /// a proxy structure for Circle
    #[derive(Debug, PartialEq, Clone)]
    pub struct CircleProps {
        pub center: Vector2,
        pub radius: f64,
        pub stroke_color: Color,
        pub stroke_width: u8,
        pub fill: Color,
    }
    
    #[derive(Debug, PartialEq, Clone)]
    pub enum Props {
        Line(LineProps),
        Rect(RectProps),
        Circle(CircleProps),
    }
}

/// # Canvas
/// a structure to hold drawables in a canvas.
/// The canvas should do all the operations of creating and manipulating drawables.
///
/// # Examples
/// ```
/// use program_core::{Canvas, Vector2, BLUE};
///
/// let mut canvas = Canvas::new(1920, 1080);
///
/// // add a line, the arguments have the same order as Line2D::new
/// canvas.add_line(Vector2::new(1.0, 1.0), Vector2::new(100.0, 100.0), Some(BLUE), Some(10), None);
///
/// // export to SVG
/// canvas.export("file.svg");
///
/// ```
#[derive(Debug)]
pub struct Canvas {
    drawables: Vec<Drawable>,
    width: u16,
    height: u16,
    selected_drawable: Option<usize>,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Canvas {
        Canvas {
            drawables: vec![],
            width,
            height,
            selected_drawable: None,
        }
    }

    pub fn add_line(
        &mut self,
        start: Vector2,
        end: Vector2,
        stroke_color: Option<Color>,
        stroke_width: Option<u8>,
        fill: Option<Color>,
    ) {
        self.drawables.push(Drawable::Line(Line2D::new(
            start,
            end,
            stroke_color,
            stroke_width,
            fill,
        )));
    }

    pub fn add_circle(
        &mut self,
        center: Vector2,
        radius: f64,
        stroke_color: Option<Color>,
        stroke_width: Option<u8>,
        fill: Option<Color>,
    ) {
        self.drawables.push(Drawable::Circle(Circle::new(
            center,
            radius,
            stroke_color,
            stroke_width,
            fill,
        )));
    }

    pub fn add_rect(
        &mut self,
        start: Vector2,
        end: Vector2,
        stroke_color: Option<Color>,
        stroke_width: Option<u8>,
        fill: Option<Color>,
    ) {
        self.drawables.push(Drawable::Rect2(Rect2::new(
            start,
            end,
            stroke_color,
            stroke_width,
            fill,
        )));
    }

    pub fn select_drawable_at(&mut self, pos: Vector2) -> bool {
        // TODO: find a better way to do this
        for (index, drawable) in self.drawables.iter().enumerate() {
            match drawable {
                Drawable::Line(line) => {
                    if line.contains(pos) {
                        self.selected_drawable = Some(index);
                        return true;
                    }
                }

                Drawable::Circle(circle) => {
                    if circle.contains(pos) {
                        self.selected_drawable = Some(index);
                        return true;
                    }
                }

                Drawable::Rect2(rect) => {
                    if rect.contains(pos) {
                        self.selected_drawable = Some(index);
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn get_selected_drawable_properties(&self) -> Result<props::Props, ()> {
        match self.selected_drawable {
            Some(index) => match &self.drawables[index] {
                Drawable::Line(line) => Ok(props::Props::Line(props::LineProps {
                    start: line.start(),
                    end: line.end(),
                    angle: line.angle(),
                    len: line.len(),
                    stroke_color: line.stroke_color().clone(),
                    stroke_width: line.stroke_width().clone(),
                    fill: line.fill(),
                })),

                Drawable::Circle(circle) => Ok(props::Props::Circle(props::CircleProps {
                    center: circle.center().clone(),
                    radius: circle.radius(),
                    stroke_color: circle.stroke_color().clone(),
                    stroke_width: circle.stroke_width(),
                    fill: circle.fill().clone(),
                })),

                Drawable::Rect2(rect) => Ok(props::Props::Rect(props::RectProps {
                    start: rect.start().clone(),
                    end: rect.end().clone(),
                    angle: rect.angle(),
                    stroke_color: rect.stroke_color().clone(),
                    stroke_width: rect.stroke_width(),
                    fill: rect.fill().clone(),
                })),
                _ => Err(()),
            },
            None => Err(()),
        }
    }

    pub fn translate_selected_drawable(&mut self, offset: Vector2) -> bool {
        if let Some(index) = self.selected_drawable {
            let selected_drawable = &mut self.drawables[index];
            match selected_drawable {
                Drawable::Line(line) => {
                    line.translate(offset);
                }
                Drawable::Circle(circle) => {
                    circle.translate(offset);
                }
                Drawable::Rect2(rect) => {
                    rect.translate(offset);
                }
                _ => {
                    return false;
                }
            }
            return true;
        }
        false
    }

    pub fn rotate_selected_drawable(&mut self, angle: f64) -> bool {
        if let Some(index) = self.selected_drawable {
            let selected_drawable = &mut self.drawables[index];
            match selected_drawable {
                Drawable::Line(line) => {
                    line.rotate(angle);
                }
                Drawable::Circle(circle) => {
                    circle.rotate(angle);
                }
                Drawable::Rect2(rect) => {
                    rect.rotate(angle);
                }
                _ => {
                    return false;
                }
            }
            return true;
        }
        false
    }

    pub fn scale_selected_drawable(&mut self, c: f64) -> bool {
        if let Some(index) = self.selected_drawable {
            let selected_drawable = &mut self.drawables[index];
            match selected_drawable {
                Drawable::Line(line) => {
                    line.scale(c);
                }
                Drawable::Circle(circle) => {
                    circle.scale(c);
                }
                Drawable::Rect2(rect) => {
                    rect.scale(c);
                }
                _ => {
                    return false;
                }
            }
            return true;
        }
        false
    }

    // TODO: test me, please :3
    pub fn export(&self, file_path: &str) -> Result<(), Error> {
        use std::fs::write;

        let mut contents =
            format!("<svg width=\"{}\" height=\"{}\">", self.width, self.height).to_string();
        for drawable in &self.drawables {
            match drawable {
                Drawable::Line(line) => {
                    contents += line.to_svg_tag().as_str();
                }
                Drawable::Circle(circle) => {
                    contents += circle.to_svg_tag().as_str();
                }
                Drawable::Rect2(rect) => {
                    contents += rect.to_svg_tag().as_str();
                }
                _ => {}
            }
        }
        contents += "</svg>";

        return write(file_path, contents);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use core::f64::EPSILON;

    #[test]
    fn test_select_drawable_at() {
        let mut canvas = Canvas::new(1920, 1080);
        canvas.add_circle(Vector2::new(960.0, 540.0), 540.0, None, None, None);

        assert!(
            canvas.select_drawable_at(Vector2::new(1000.0, 600.0)),
            "Testing selection in a circle"
        );

        let circle_props = canvas.get_selected_drawable_properties();

        match circle_props {
            Ok(props::Props::Circle(props)) => {
                assert_eq!(
                    Vector2::new(960.0, 540.0),
                    props.center,
                    "Testing circle props"
                );
                assert_eq!(540.0, props.radius, "Testing circle props");
            }
            _ => {
                panic!("did not return the properties of the circle");
            }
        }

        assert!(
            !canvas.select_drawable_at(Vector2::new(1900.0, 600.0)),
            "Testing selection in an empty space"
        );
    }

    #[test]
    fn test_translate_selected_drawable() {
        let mut canvas = Canvas::new(1920, 1080);
        canvas.add_circle(Vector2::new(960.0, 540.0), 540.0, None, None, None);

        assert!(
            canvas.select_drawable_at(Vector2::new(1000.0, 600.0)),
            "Selecting the circle"
        );
        assert!(
            canvas.translate_selected_drawable(Vector2::new(1.0, 1.0)),
            "Translating the circle"
        );

        match canvas.get_selected_drawable_properties() {
            Ok(props::Props::Circle(props)) => {
                assert_eq!(
                    Vector2::new(961.0, 541.0),
                    props.center,
                    "Testing shifted center"
                );
            }

            _ => {
                panic!("did not return the properties of the circle");
            }
        }
    }

    #[test]
    fn test_rotate_selected_drawable() {
        let mut canvas = Canvas::new(1920, 1080);
        canvas.add_rect(
            Vector2::new(200.0, 200.0),
            Vector2::new(300.0, 500.0),
            None,
            None,
            None,
        );

        assert!(
            canvas.select_drawable_at(Vector2::new(250.0, 350.0)),
            "Selecting the rectangle"
        );
        assert!(
            canvas.rotate_selected_drawable(90.0),
            "Rotating the rectangle"
        );

        match canvas.get_selected_drawable_properties() {
            Ok(props::Props::Rect(props)) => {
                assert_eq!(90.0, props.angle, "Testing rotated rectangle");
            }

            _ => {
                panic!("did not return the properties of the circle");
            }
        }
    }

    #[test]
    fn test_scale_selected_drawable() {
        let mut canvas = Canvas::new(1920, 1080);
        canvas.add_circle(Vector2::new(200.0, 200.0), 1.0, None, None, None);

        assert!(
            canvas.select_drawable_at(Vector2::new(200.0, 200.0)),
            "Selecting the circle"
        );
        assert!(canvas.scale_selected_drawable(2.0), "Scaling the circle");

        match canvas.get_selected_drawable_properties() {
            Ok(props::Props::Circle(props)) => {
                assert_eq!(2.0, props.radius, "Testing scaled circle");
            }

            _ => {
                panic!("did not return circle properties");
            }
        }
    }
}
