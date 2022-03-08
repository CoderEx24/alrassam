//! # drawable
//! this module will contain code for drawable objects
//! like lines, circles, rectangles, etc.

use std::collections::HashMap;

/// # Draw
/// A trait for drawable objects.
/// this trait will contain methods that helps
/// construct an SVG tag for the drawable object
pub trait Draw {
    fn get_svg_tag_name() -> String;
    fn get_svg_tag_propeties(self: &Self) -> HashMap<String, String>;
}

pub mod line2d;
pub mod point2d;
pub mod circle;
pub mod text;

