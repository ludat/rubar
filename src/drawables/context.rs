use std::fmt;

use cairo::*;
use pango::FontDescription;

use draw::{Drawable, Size, Color};
use window::Window;

pub struct Config {
    pub font: FontDescription,
    pub color: Color,
    pub alpha: f64,
}

impl Config {
    pub fn new(font: &str, color: Color, alpha: f64) -> Config {
        Config {
            font: FontDescription::from_string(font),
            color: color,
            alpha: alpha,
        }
    }
}

pub struct Context {
    pub font: Option<FontDescription>,
    pub color: Option<Color>,
    pub alpha: Option<f64>,
    pub children: Vec<Box<Drawable>>,
}

impl Drawable for Context {
    unsafe fn _draw(&self, w: &mut Window, c: &Config) -> Size {
        let c = self.derive(c);
        self.children.iter().fold(Size::empty(),
            |size, d| {
                cairo_set_source_rgb(
                    w.context, c.color.red, c.color.green, c.color.blue);
                let s = d._draw(w, &c);
                cairo_rel_move_to(w.context, s.width as f64, 0.0);
                s + size
            }
        )
    }
}

impl Context {
    pub fn empty() -> Context {
        Context {
            font: None,
            color: None,
            alpha: None,
            children: Vec::new(),
        }
    }
    pub fn derive(&self, c: &Config) -> Config {
        // TODO impement some kind of smart pointer here because this is ugly
        Config {
            font: self.font.clone().unwrap_or(c.font.clone()),
            color: self.color.unwrap_or(c.color),
            alpha: self.alpha.unwrap_or(c.alpha),
        }
    }
    pub fn push(&mut self, d: Box<Drawable>) -> &mut Context {
        self.children.push(d);
        self
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context")
    }
}
