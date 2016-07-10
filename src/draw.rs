use std::ops::{Add, Sub};

use window::Window;
use drawables::{Config, Context, Image};

#[derive(Debug, PartialEq, Clone)]
pub enum Draw {
    Text(String),
    Context(Context),
    Image(Image)
}

impl Drawable for Draw {
    fn draw(&self, window: &mut Window, config: &Config, position: Position) -> Size {
            match *self {
                Draw::Text(ref t) => t.draw(window, config, position),
                Draw::Context(ref c) => c.draw(window, config, position),
                Draw::Image(ref i) => i.draw(window, config, position),
            }
    }
}

pub trait Drawable {
    fn draw(&self, w: &mut Window, c: &Config, p: Position) -> Size;
    // fn size(&self)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn origin() -> Position {
        Position {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl Sub<Size> for Position {
    type Output = Position;
    fn sub(self, other: Size) -> Position {
        Position {
            x: self.x - other.width as f64,
            y: self.y - other.height as f64,
        }
    }
}

impl Add<Size> for Position {
    type Output = Position;
    fn add(self, other: Size) -> Position {
        Position {
            x: self.x + other.width as f64,
            y: self.y + other.height as f64,
        }
    }
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
    fn add(self, other: Size) -> Size {
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
