use std::ffi::CString;

use cairo::*;
use pango::*;
use pangocairo_gen::*;

use window::Window;

use drawables::Context;
use draw::Drawable;
use draw::Size;

// pub struct Text<'a> {
//     content: &'a str,
//     font: *mut PangoFontDescription,
// }

// impl<'a> Text<'a> {
//     pub fn new(s: &str) -> Text {
//         Text {
//             content: s,
//             font: unsafe { pango_font_description_from_string(
//                 CString::new("Terminess 12").unwrap().as_ptr()) }
//         }
//     }

//     pub fn font(&'a mut self, s: &str) -> &'a mut Text {
//         self.font = unsafe { pango_font_description_from_string(
//             CString::new(s).unwrap().as_ptr()) };
//         self
//     }
// }

impl<'a> Drawable for &'a str {
    unsafe fn _draw(&self, w: &mut Window, c: &Context) -> Size {
        // cairo_select_font_face (w.context, "Georgia",
        //                                CAIRO_FONT_SLANT_NORMAL,
        //                                CAIRO_FONT_WEIGHT_BOLD);
        // cairo_set_font_size (w.context, 1.2);
        // cairo_show_text (w.context, "a");

        let layout = pango_cairo_create_layout(w.context);
        pango_layout_set_text(
            layout, CString::new(*self).unwrap().as_ptr(), -1);

        pango_layout_set_font_description(layout, c.font);

        // cairo_set_source_rgb(w.context, 1.0, 1.0, 1.0);
        // cairo_set_source_rgb(w.context, c.color.0, c.color.1, c.color.2);
        pango_cairo_update_layout(w.context, layout);

        let mut size: Size = Size::empty();

        pango_layout_get_pixel_size(layout,
                                    &mut size.width as *mut i32,
                                    &mut size.height as *mut i32);

        println!("drawing: {:?}", self);
        println!("height: {}, width {}", size.height, size.width);

        pango_cairo_show_layout(w.context, layout);

        // glib::g_object_unref(layout);

        size
    }
}
