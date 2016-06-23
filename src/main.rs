extern crate cairo;
extern crate x11;

use x11::xlib::*;

use cairo::ffi::cairo_xlib_surface_create;

use std::ptr::{
    null,
    null_mut,
};

use std::os::raw::c_uint;

const WIDTH: c_uint = 640;
const HEIGHT: c_uint = 480;

fn main() {
    println!("Start");
    unsafe {
        let dsp = XOpenDisplay(null());

        if dsp == null_mut() { panic!("Failed to open display"); }

        let screen = XDefaultScreen(dsp);
        let root = XRootWindow(dsp, screen);
        let window = XCreateSimpleWindow(dsp, root,
                                         0, 0, WIDTH, HEIGHT, 0, 0, 0);

        // XSelectInput(dsp, window, ButtonPressMask | KeyPressMask);
        XMapWindow(dsp, window);
        let dv = XDefaultVisual(dsp, screen);

        let sfc = cairo_xlib_surface_create(
            dsp,
            window,
            dv,
            WIDTH as i32,
            HEIGHT as i32
        );
    }
    println!("END");
    // cairo_xlib_surface_set_size(sfc, x, y);

}
