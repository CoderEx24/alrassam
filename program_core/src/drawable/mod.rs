//! # drawable
//! this module will contain code for drawable objects
//! like lines, circles, rectangles, etc.

use std::collections::HashMap;
use std::string::ToString;

/// # Draw
/// A trait for drawable objects.
/// this trait will contain methods that helps
/// construct an SVG tag for the drawable object
pub trait Draw {
    // allow method chaining
    fn translate(self: &mut Self, offset: vector::Vector2) -> &mut Self;
    fn rotate(self: &mut Self, angle: f64) -> &mut Self;
    fn scale(self: &mut Self, c: f64) -> &mut Self;

    fn contains(self: &Self, point: vector::Vector2) -> bool;

    fn get_svg_tag_name(self: &Self) -> String;
    fn get_svg_tag_properties(self: &Self) -> HashMap<String, String>;
    fn get_svg_inner_content(self: &Self) -> Option<String>;
    fn to_svg_tag(self: &Self) -> String {
        let mut svg_tag = format!("<{}", self.get_svg_tag_name());

        // TODO: Use iterators instead
        for (key, val) in self.get_svg_tag_properties().iter() {
            svg_tag += format!(" {}=\"{}\"", key, val).as_str();
        }

        // TODO: there must be a better way to do this >:(
        match self.get_svg_inner_content() {
            Some(txt) => {
                svg_tag += format!("> {} <{}/>", txt, self.get_svg_tag_name()).as_str();
            }
            None => {
                svg_tag += "/>";
            }
        }

        svg_tag
    }
}

/// # Color
/// A structure to hold color information.
/// it consists of 4 fields:- (r, g, b, a)
#[derive(PartialEq, Clone, Debug)]
pub struct Color(pub u8, pub u8, pub u8, pub f32);

impl ToString for Color {
    fn to_string(&self) -> String {
        let (r, g, b, a) = (self.0, self.1, self.2, self.3);
        format!("rgba({}, {}, {}, {})", r, g, b, a)
    }
}

/// # color_from_hex
/// constructs a color from a number
///
/// # Examples
/// ```
/// use program_core::{Color, color_from_hex};
///
/// let red_number   = 0xff0000;
/// let green_number = 0x00ff00;
/// let blue_number  = 0x0000ff;
///
/// assert_eq!(Color(255, 0  , 0  , 0.0), color_from_hex(red_number, 0.0));
/// assert_eq!(Color(0  , 255, 0  , 0.5), color_from_hex(green_number, 0.5));
/// assert_eq!(Color(0  , 0  , 255, 0.7), color_from_hex(blue_number, 0.7));
/// ```
pub fn color_from_hex(hexnumber: u64, alpha: f32) -> Color {
    Color(
        ((hexnumber & 0xff0000) >> 16) as u8,
        ((hexnumber & 0x00ff00) >> 8) as u8,
        (hexnumber & 0x0000ff) as u8,
        alpha,
    )
}

pub const RED: Color = Color(255, 0, 0, 1.0);
pub const GREEN: Color = Color(0, 255, 0, 1.0);
pub const BLUE: Color = Color(0, 0, 255, 1.0);
pub const BLACK: Color = Color(255, 255, 255, 1.0);
pub const WHITE: Color = Color(0, 0, 0, 1.0);

pub mod circle;
pub mod line2d;
pub mod vector;
pub mod canvas;
pub mod text;
pub mod rect2d;
