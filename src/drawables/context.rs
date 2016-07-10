use cairo::*;
use pango::FontDescription;

use draw::{Draw, Drawable, Size, Position, Color};
use window::Window;

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
pub struct Context {
    pub font: Option<FontDescription>,
    pub color: Option<Color>,
    pub alpha: Option<f64>,
    pub children: Vec<Draw>,
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
    pub fn push(mut self, d: Draw) -> Context {
        self.children.push(d);
        self
    }
    pub fn color(mut self, color: Color) -> Context {
        self.color = Some(color);
        self
    }
    pub fn font(mut self, font: FontDescription) -> Context {
        self.font = Some(font);
        self
    }
    pub fn alpha(mut self, alpha: f64) -> Context {
        self.alpha = Some(alpha);
        self
    }
}

impl Drawable for Context {
    fn draw(&self, w: &mut Window, c: &Config, p: Position) -> Size {
        let c: Config = self.derive(c);
        self.children.iter().fold(Size::empty(),
            |size, d| {
                unsafe { cairo_set_source_rgb(
                    w.context, c.color.red, c.color.green, c.color.blue) }
                let s = d.draw(w, &c, p + size);
                size + s
            }
        )
    }
}
