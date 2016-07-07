use std::ffi::CString;
use std::ops::{Add,Drop};

use cairo::*;
use pango::*;
// use pangocairo_gen::*;

use draw::{Drawable, Size};
use window::Window;

pub struct Context {
    pub font: *mut PangoFontDescription,
    pub color: (f64, f64, f64),
    pub alpha: f64,
    pub children: Vec<Box<Drawable>>,
}

impl Context {
    pub fn new(font: &str, color: &str, alpha: f64) -> Context {
        Context {
            font: unsafe { pango_font_description_from_string(
                CString::new(font).unwrap().as_ptr()) },
            children: vec![],
            color: (1.0, 1.0, 1.0),
            alpha: alpha,
        }
    }

    pub fn add(&mut self, d: Box<Drawable>) -> &mut Context {
        self.children.push(d);
        self
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            pango_font_description_free(self.font);
        }
    }
}

impl Drawable for Context {
    unsafe fn _draw(&self, w: &mut Window, _: &Context) -> Size {

        cairo_set_source_rgb(
            w.context, self.color.0, self.color.1, self.color.2);

        self.children.iter().fold(Size::empty(), |size, d| {
            let s = d._draw(w, self);
            cairo_rel_move_to(w.context, s.width as f64, 0.0);
            s + size
        })
    }
}
