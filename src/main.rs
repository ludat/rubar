extern crate cairo;
extern crate x11;
extern crate glib;

use x11::xlib::*;

use cairo::ffi::cairo_xlib_surface_create;
use cairo::ffi::cairo_surface_t;
use cairo::{Context, Surface};

use std::ptr::{
    null,
    null_mut,
};

use glib::translate::FromGlibPtr;
use std::os::raw::c_uint;

const WIDTH: c_uint = 640;
const HEIGHT: c_uint = 480;

fn main() {
    println!("Start");
    let dsp = unsafe { XOpenDisplay(null()) };

    if dsp == null_mut() { panic!("Failed to open display"); }

    let screen = unsafe { XDefaultScreen(dsp) };
    let root = unsafe { XRootWindow(dsp, screen) };
    let window = unsafe { XCreateSimpleWindow(
        dsp, root, 0, 0, WIDTH, HEIGHT, 0, 0, 0)};

    unsafe {
        // XSelectInput(dsp, window, ButtonPressMask | KeyPressMask);
        XMapWindow(dsp, window);
    }

    let dv = unsafe { XDefaultVisual(dsp, screen) };

    let _sfc: *mut cairo_surface_t = unsafe { cairo_xlib_surface_create(
        dsp,
        window,
        dv,
        WIDTH as i32,
        HEIGHT as i32
    ) };
    let sfc: Surface = unsafe { FromGlibPtr::from_glib_full(_sfc) };
    let cr = Context::new(&sfc);

    cr.set_line_width(0.1);
    cr.set_source_rgb(255.0, 0.0, 0.0);
    cr.rectangle(0.25, 0.25, 0.5, 0.5);
    cr.stroke();

    std::thread::sleep_ms(5000);

    println!("END");
    // cairo_xlib_surface_set_size(sfc, x, y);

}
