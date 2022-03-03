pub trait Drawable {}

#[derive(PartialEq, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

pub struct Line {
    start: Point,
    end: Point,
    len: f64,
    angle: f64,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line {
            start,
            end,
            len: ((start.x - end.x).powi(2) + (start.y - end.y).powi(2)).sqrt(),
            angle: ((start.y - end.y) / (start.x - end.x)).atan()
        }
    }
}

