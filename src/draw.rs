use std::ops::Add;

use window::Window;
use drawables::Context;

pub trait Drawable {
    unsafe fn _draw(&self, w: &mut Window, c: &Context) -> Size;
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

#[derive(Debug,Copy,Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn white() -> Color {
        Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }
    pub fn black() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color {
            red: red,
            green: green,
            blue: blue,
        }
    }
}
