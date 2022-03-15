use program_core::{Drawable, Point};

#[derive(Clone, PartialEq)]
pub enum Message {
    Line,
    FinishLine(Point),
}

#[derive(Clone, PartialEq)]
pub struct AppState {
    drawables: Box<Vec<Drawable>>,
    current_message: Option<Message>,
    prev_message: Option<Message>,
    current_point: Option<Point>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState { 
            drawables: Box::new(Vec::new()),
            current_message: None,
            prev_message: None,
            current_point: None,
        }
    }

    pub fn add(&mut self, drawable: &Drawable) {
        (*self.drawables).push(drawable.clone());

    }

    pub fn drawables(&self) -> &Vec<Drawable> {
        self.drawables.as_ref()
    }

    pub fn current_message(&self) -> Option<Message> {
        self.current_message.clone()
    }
    
    pub fn prev_message(&self) -> Option<Message> {
        self.prev_message.clone()
    }

    pub fn set_message(&mut self, new_message: Option<Message>) {
        self.prev_message = self.current_message.clone();
        self.current_message = new_message;
    }

}


