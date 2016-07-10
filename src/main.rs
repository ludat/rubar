extern crate cairo_sys as cairo;
extern crate pango;
extern crate pango_sys;
extern crate glib;
extern crate gobject_sys as gobject;
extern crate gdk_pixbuf;
extern crate gdk_sys as gdk;
extern crate x11;

#[macro_use]
extern crate nom;

use std::io;
use std::io::BufRead;

mod pangocairo_gen;
use nom::IResult;

mod drawables;
mod window;
mod draw;
mod parser;
use draw::Color;
use window::Window;
use draw::{Drawable, Position};
use drawables::{Image, Config, Context};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 20;

fn main() {
    let mut w = Window::new("title", WIDTH as u32, HEIGHT as u32);
    w.clear(Color::black());
    w.flush();

    let stdin = io::stdin();
    let root = Config::new("Terminus 10", Color::white(), 0.0);
    for line in stdin.lock().lines() {
        let mut base = Context::empty();
        match parser::draw(&line.unwrap().into_bytes()) {
            IResult::Done(b"", children) => {
                use std::path::Path;
                use draw::Draw;
                base.children = vec![
                    Draw::Image(
                        Image::new(
                            Path::new("/home/ludat/Pictures/icon.png")
                                .to_path_buf()
                        ).unwrap()
                    ),
                    Draw::Image(
                        Image::new(
                            Path::new("/home/ludat/Pictures/icon.png")
                                .to_path_buf()
                        ).unwrap()
                    ),
                    Draw::Image(
                        Image::new(
                            Path::new("/home/ludat/Pictures/icon.png")
                                .to_path_buf()
                        ).unwrap()
                    ),
                    Draw::Image(
                        Image::new(
                            Path::new("/home/ludat/Pictures/icon.png")
                                .to_path_buf()
                        ).unwrap()
                    ),
                    Draw::Image(
                        Image::new(
                            Path::new("/home/ludat/Pictures/icon.png")
                                .to_path_buf()
                        ).unwrap()
                    ),
                    Draw::Image(
                        Image::new(
                            Path::new("/home/ludat/Pictures/icon.png")
                                .to_path_buf()
                        ).unwrap()
                    ),
                ];
                w.clear(Color::black());
                println!("{:?}", base.children);
                base.draw(&mut w, &root, Position::origin());
                w.flush();
            },
            _ => { println!("Parsing failed") },
        }
    }
}
