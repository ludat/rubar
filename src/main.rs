extern crate cairo_sys as cairo;
extern crate pango;
extern crate pango_sys;
extern crate glib;
extern crate gobject_sys as gobject;
extern crate x11;
extern crate combine;

use std::io;
use std::io::BufRead;

use cairo::*;
mod pangocairo_gen;
mod drawables;
mod window;
mod draw;
use draw::Color;
use window::Window;
use draw::Drawable;
use drawables::{Context, ContextBuilder};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 20;

fn main() {
    let mut w = Window::new("title", WIDTH as u32, HEIGHT as u32);


    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let root = Context::new("Terminus 10", Color::white(), 0.0);
        let mut base = ContextBuilder::empty();
        base.push(Box::new(line.unwrap()));
        w.clear();
        unsafe {
            cairo_move_to(w.context, 0.0, 0.0);
            base._draw(&mut w, &root);
        }
        w.flush();
    }
}
