extern crate cairo_sys as cairo;
extern crate pango;
extern crate pango_sys;
extern crate glib;
extern crate gobject_sys as gobject;
extern crate x11;

#[macro_use]
extern crate nom;

use std::io;
use std::io::BufRead;

use cairo::*;
mod pangocairo_gen;
use nom::IResult;

mod drawables;
mod window;
mod draw;
mod parser;
use draw::Color;
use window::Window;
use draw::Drawable;
use drawables::{Config, Context};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 20;

fn main() {
    let mut w = Window::new("title", WIDTH as u32, HEIGHT as u32);
    w.clear(Color::black());
    w.flush();

    // let thing: IResult<&str, Vec<Box<Drawable>>> = drawables("leleel {bla}");
    // let thing = parser::drawables("leleel {} ble");
    // println!("the thing: {:?}", thing);

    let stdin = io::stdin();
    let root = Config::new("Terminus 10", Color::white(), 0.0);
    for line in stdin.lock().lines() {
        let mut base = Context::empty();
        if let IResult::Done(_, children) = parser::drawables(&line.unwrap().into_bytes()) {
            println!("{:?}", children);
            base.children = children;
            w.clear(Color::black());
            unsafe {
                cairo_move_to(w.context, 0.0, 0.0);
                base._draw(&mut w, &root);
            }
            w.flush();
        } else {
            println!("Failed parsing");
        };
    }
}
