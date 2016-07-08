use std::ffi::CString;
use std::ptr::{null, null_mut};
use std::mem::zeroed;
use std::os::raw::{c_uint, c_uchar};
use x11::xlib;
use cairo::*;

/// Provides a basic framework for connecting to an X Display,
/// creating a window, displaying it and running the event loop
pub struct Window {
    pub display: *mut xlib::Display,
    pub window: xlib::Window,
    pub screen: i32,
    pub width: u32,
    pub height: u32,
    pub surface: *mut cairo_surface_t,
    pub context: *mut cairo_t,
}

impl Window {
    /// Create a new window with a given title and size
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        unsafe {
            // Open display
            let display = xlib::XOpenDisplay(null());
            if display == null_mut() {
                panic!("can't open display");
            }

            // Load atoms
            let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();
            let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();

            let wm_delete_window = xlib::XInternAtom(
                display, wm_delete_window_str.as_ptr(), xlib::False);
            let wm_protocols = xlib::XInternAtom(
                display, wm_protocols_str.as_ptr(), xlib::False);

            if wm_delete_window == 0 || wm_protocols == 0 {
                panic!("can't load atoms");
            }

            // Create window
            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);

            let mut attributes: xlib::XSetWindowAttributes = zeroed();
            attributes.override_redirect = xlib::True;
            attributes.background_pixel = xlib::XWhitePixel(display, screen);

            let window = xlib::XCreateWindow(
                display,
                root,
                0, 0,
                width as c_uint,
                height as c_uint,
                0, 0,
                xlib::InputOutput as c_uint,
                null_mut(),
                xlib::CWBackPixel,
                &mut attributes
            );

            // Set window title
            let title_str = CString::new(title).unwrap();
            xlib::XStoreName(display, window, title_str.as_ptr() as *mut _);

            // Subscribe to delete (close) events
            let mut protocols = [wm_delete_window];

            if xlib::XSetWMProtocols(
                display, window, &mut protocols[0] as *mut xlib::Atom,
                1) == xlib::False {
                panic!("can't set WM protocols");
            }

            println!("before");

            let net_wm_window_type = CString::new("_NET_WM_WINDOW_TYPE").unwrap();
            let net_wm_window_type_dock = CString::new("_NET_WM_WINDOW_TYPE_DOCK").unwrap();
            let net_wm_state = CString::new("_NET_WM_STATE").unwrap();
            let net_wm_state_above = CString::new("_NET_WM_STATE_ABOVE").unwrap();
            let net_wm_state_sticky = CString::new("_NET_WM_STATE_STICKY").unwrap();
            let net_wm_strut = CString::new("_NET_WM_STRUT").unwrap();
            let net_wm_strut_partial = CString::new("_NET_WM_STRUT_PARTIAL").unwrap();
            let cardinal = CString::new("CARDINAL").unwrap();
            let atom = CString::new("ATOM").unwrap();


            xlib::XChangeProperty(
                display, window,
                xlib::XInternAtom(
                    display, net_wm_window_type.as_ptr(), xlib::False),
                xlib::XInternAtom(
                    display, atom.as_ptr(), xlib::False),
                32, xlib::PropModeReplace,
                &xlib::XInternAtom(
                    display,
                    net_wm_window_type_dock.as_ptr(),
                    xlib::False) as *const u64 as *const c_uchar,
                // thd as *const c_uchar,
                1
            );

            xlib::XChangeProperty(
                display, window,
                xlib::XInternAtom(
                    display,
                    net_wm_state.as_ptr(),
                    xlib::False),
                xlib::XInternAtom(
                    display,
                    atom.as_ptr(),
                    xlib::False),
                32,
                xlib::PropModeReplace,
                &xlib::XInternAtom(
                    display,
                    net_wm_state_above.as_ptr(),
                    xlib::False) as *const u64 as *const c_uchar,
                1
            );

            xlib::XChangeProperty(
                display,
                window,
                xlib::XInternAtom(
                    display,
                    net_wm_state.as_ptr(),
                    xlib::False),
                xlib::XInternAtom(
                    display,
                    atom.as_ptr(),
                    xlib::False),
                32,
                xlib::PropModeAppend,
                &xlib::XInternAtom(
                    display,
                    net_wm_state_sticky.as_ptr(),
                    xlib::False) as *const u64 as *const c_uchar,
                1
            );

            let strut_p: &[u64; 12] = &[
                0,            0, height as u64, 0,
                0,            0,             0, 0,
                0, width as u64,             0, 0 ];
            xlib::XChangeProperty(
                display,
                window,
                xlib::XInternAtom(
                    display,
                    net_wm_strut_partial.as_ptr(),
                    xlib::False),
                xlib::XInternAtom(
                    display,
                    cardinal.as_ptr(),
                    xlib::False),
                32,
                xlib::PropModeReplace,
                strut_p.as_ptr() as *const c_uchar,
                12
            );

            let strut: &[u64; 4] = &[ 0, 0, 25, 0 ];
            xlib::XChangeProperty(
                display,
                window,
                xlib::XInternAtom(
                    display,
                    net_wm_strut.as_ptr(),
                    xlib::False),
                xlib::XInternAtom(
                    display,
                    cardinal.as_ptr(),
                    xlib::False),
                32,
                xlib::PropModeReplace,
                strut.as_ptr() as *const c_uchar,
                4
            );

            // show the window on the screen
            xlib::XMapWindow(display, window);

            let cairo_surface: *mut cairo_surface_t =
                cairo_xlib_surface_create(
                    display,
                    window,
                    xlib::XDefaultVisual(display, screen),
                    width as i32,
                    height as i32,
                );
            // cairo_xlib_surface_set_size(sfc, 2000, 1000);
            let cairo_context = cairo_create(cairo_surface);

            Window {
                display: display,
                window: window,
                screen: screen,
                width: width,
                height: height,
                surface: cairo_surface,
                context: cairo_context
            }
        }
    }

    /// Flush the X11 buffer
    pub fn flush(&mut self) {
        unsafe {
            xlib::XFlush(self.display);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            cairo_set_source_rgb(self.context, 0.0, 0.0, 0.0);
            cairo_rectangle (
                self.context, 0.0, 0.0, self.width as f64, self.height as f64);
            cairo_fill(self.context);
        }
    }
}

impl Drop for Window {
    /// Destroys the window and disconnects from the display
    fn drop(&mut self) {
        unsafe {
            cairo_surface_destroy(self.surface);
            cairo_destroy(self.context);
            xlib::XDestroyWindow(self.display, self.window);
            xlib::XCloseDisplay(self.display);
        }
    }
}
