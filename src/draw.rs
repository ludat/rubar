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
