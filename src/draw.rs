use std::ops::Add;

use window::Window;
use drawables::{Config, Context};

#[derive(Debug, PartialEq, Clone)]
pub enum Draw {
    Text(String),
    Context(Context),
}

impl Drawable for Draw {
    fn draw(&self, window: &mut Window, config: &Config) -> Size {
            match *self {
                Draw::Text(ref t) => t.draw(window, config),
                Draw::Context(ref c) => c.draw(window, config)
            }
    }
}

pub trait Drawable {
    fn draw(&self, w: &mut Window, c: &Config) -> Size;
    // fn size(&self)
}

#[derive(Debug,Copy,Clone)]
pub struct Size {
    pub height: i32,
    pub width: i32,
}

impl Size {
    pub fn empty() -> Size {
        Size {
            height: 0,
            width: 0,
        }
    }
}

impl Add for Size {
    type Output = Size;
    fn add(self, other: Self) -> Self {
        Size {
            height: self.height,
            width: self.width + other.width,
        }
    }
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color {
            red: red,
            green: green,
            blue: blue,
        }
    }

    pub fn white() -> Color {
        Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }
    pub fn grey() -> Color {
        Color {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
        }
    }
    pub fn black() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn red() -> Color {
        Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        }
    }
    pub fn green() -> Color {
        Color {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
        }
    }
    pub fn blue() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
        }
    }
    pub fn cyan() -> Color {
        Color {
            red: 0.0,
            green: 1.0,
            blue: 1.0,
        }
    }
    pub fn yellow() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
        }
    }
    pub fn purple() -> Color {
        Color {
            red: 0.5,
            green: 0.0,
            blue: 0.5,
        }
    }
}
