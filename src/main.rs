extern crate cairo;
extern crate x11;
extern crate glib;

use x11::xlib::*;

use std::f64::consts::PI;
use cairo::ffi::*;
use cairo::{Context, Surface};
use cairo::enums::{FontSlant, FontWeight};

use std::ptr::{
    null,
    null_mut,
};
use std::mem::zeroed;

use glib::translate::FromGlibPtr;
use std::os::raw::c_uint;

const WIDTH: c_uint = 640;
const HEIGHT: c_uint = 480;

fn main() {
    unsafe {
        println!("Start");
        let display = XOpenDisplay(null());

        if display == null_mut() { panic!("Failed to open display"); }

        let screen = XDefaultScreen(display);
        let root = XRootWindow(display, screen);
        let white_pixel = XWhitePixel(display, screen);

        let mut attributes: XSetWindowAttributes = zeroed();
        attributes.background_pixel = white_pixel;

        let window = XCreateWindow(
            display, // display
            root, // root window
            0, // x position
            0, // y position
            WIDTH, // width
            HEIGHT, //height
            0, // border width
            XDefaultDepth(display, screen), // depth
            CopyFromParent as c_uint, // class
            XDefaultVisual(display, screen),
            CWBackPixel,
            &mut attributes
        );

        // XSelectInput(display, window, ButtonPressMask | KeyPressMask);
        XMapWindow(display, window);


        let sfc: *mut cairo_surface_t = cairo_xlib_surface_create(
            display,
            window,
            XDefaultVisual(display, screen),
            WIDTH as i32,
            HEIGHT as i32
        );
        cairo_xlib_surface_set_size(sfc, 300, 200);
        let sfc: Surface = FromGlibPtr::from_glib_full(sfc);
    let cr = Context::new(&sfc);

    // println!("size: {}x{}", )
    println!("drawing");

    cr.scale(500f64, 500f64);

    cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
    cr.set_font_size(0.35);

    cr.move_to(0.04, 0.53);
    cr.show_text("Hello");

    cr.move_to(0.27, 0.65);
    cr.text_path("void");
    cr.set_source_rgb(0.5, 0.5, 1.0);
    cr.fill_preserve();
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.set_line_width(0.01);
    cr.stroke();

    cr.set_source_rgba(1.0, 0.2, 0.2, 0.6);
    cr.arc(0.04, 0.53, 0.02, 0.0, PI * 2.);
    cr.arc(0.27, 0.65, 0.02, 0.0, PI * 2.);
    cr.fill();




    std::thread::sleep_ms(2000);

}
}
