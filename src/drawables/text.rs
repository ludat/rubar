use std::ffi::CString;
use std::fmt::Debug;

use pango_sys::*;
use pangocairo_gen::*;
use gobject::*;
use glib::translate::ToGlibPtr;

use window::Window;

use drawables::Context;
use draw::Drawable;
use draw::Size;

impl<T> Drawable for T where T: AsRef<str> + Debug {
    unsafe fn _draw(&self, w: &mut Window, c: &Context) -> Size {
        let layout = pango_cairo_create_layout(w.context);
        pango_layout_set_text(
            layout, CString::new(self.as_ref()).unwrap().as_ptr(), -1);

        pango_layout_set_font_description(
            layout, (&c.font).to_glib_none().0);



        pango_cairo_update_layout(w.context, layout);

        let mut size: Size = Size::empty();

        pango_layout_get_pixel_size(layout,
                                    &mut size.width as *mut i32,
                                    &mut size.height as *mut i32);

        println!("drawing: {:?}", self.as_ref());
        println!("height: {}, width {}", size.height, size.width);

        pango_cairo_show_layout(w.context, layout);

        g_object_unref(layout as *mut GObject);

        size
    }
}
