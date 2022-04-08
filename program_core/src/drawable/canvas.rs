use super::{
    vector::Vector2, circle::Circle, line2d::Line2D, rect2d::Rect2,
    Color, Draw, super::Drawable
};

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

    use super::{ Vector2, Color };
    
    /// # LineProps
    /// a proxy structure for Line2D
    #[derive(Debug)]
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
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub struct CircleProps {
        pub center: Vector2,
        pub radius: f64,
        pub stroke_color: Color,
        pub stroke_width: u8,
        pub fill: Color,
    }

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
            width, height,
            selected_drawable: None,
        }
    }

    pub fn add_line(&mut self, start: Vector2, end: Vector2, stroke_color: Option<Color>, stroke_width: Option<u8>, fill: Option<Color>) {
        self.drawables.push(Drawable::Line(Line2D::new(start, end, stroke_color, stroke_width, fill)));
    }

    pub fn add_circle(&mut self, center: Vector2, radius: f64, stroke_color: Option<Color>, stroke_width: Option<u8>, fill: Option<Color>) {
        self.drawables.push(Drawable::Circle(Circle::new(center, radius, stroke_color, stroke_width, fill)));        
    }

    pub fn add_rect(&mut self, start: Vector2, end: Vector2, stroke_color: Option<Color>, stroke_width: Option<u8>, fill: Option<Color>) {
        self.drawables.push(Drawable::Rect2(Rect2::new(start, end, stroke_color, stroke_width, fill)));
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
                },

               Drawable::Circle(circle) => {
                    if circle.contains(pos) {
                        self.selected_drawable = Some(index);
                        return true;
                    }
               },

               Drawable::Rect2(rect) => {
                   if rect.contains(pos) {
                       self.selected_drawable = Some(index);
                       return true;
                   }
               },
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
                _ => Err(())
            },
            None => Err(()),
        }
    }
}

