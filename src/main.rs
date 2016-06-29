extern crate cairo_sys as cairo;
extern crate x11;
extern crate glib;

use std::io;
use std::io::prelude::*;
use std::f64::consts::PI;



mod window;
use window::Window;

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 20;

fn main() {
    println!("get");

    let mut w = Window::new("title", WIDTH as u32, HEIGHT as u32);


    println!("GO!");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // cr.set_source_rgb(0.0, 0.0, 0.0);
        // cr.rectangle (0.0, 0.0, WIDTH as f64, HEIGHT as f64);
        // cr.fill();

        // cr.set_source_rgb(1.0, 1.0, 1.0);
        // cr.arc(4., 53., 2., 0.0, PI * 200.);
        // cr.arc(27., 65., 2., 0.0, PI * 200.);
        // cr.fill();

        // cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        // cr.set_font_size(15.);

        // cr.move_to(4., 13.);
        // cr.show_text(&line.unwrap());

        // cr.move_to(47., 13.);
        // cr.text_path("void");
        // cr.set_source_rgb(0.5, 0.5, 1.0);
        // cr.fill_preserve();
        // cr.set_source_rgb(1.0, 1.0, 1.0);
        // cr.set_line_width(1.);
        // cr.stroke();
        // w.flush();
    }
}
