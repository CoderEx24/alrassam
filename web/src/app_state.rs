use program_core::Drawable;

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    drawables: Box<Vec<Drawable>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState { drawables: Box::new(Vec::new()) }
    }

    pub fn add(&mut self, drawable: &Drawable) {
        (*self.drawables).push(drawable.clone());

    }

    pub fn drawables(&self) -> &Vec<Drawable> {
        self.drawables.as_ref()
    }
}


