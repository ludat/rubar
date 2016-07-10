use gdk_pixbuf::Pixbuf;
use gdk::gdk_cairo_set_source_pixbuf;
use glib::translate::ToGlibPtr;
use cairo::*;

use window::Window;

use std::path::PathBuf;
use drawables::Config;
use draw::Drawable;
use draw::{Size, Position};

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    path: PathBuf,
    pixbuf: Pixbuf,
}

impl Image {
    pub fn new(path: PathBuf) -> Option<Image>  {
        if ! path.is_file() {
            None
        } else {
            path.to_str()
                .and_then(|p| Pixbuf::new_from_file_at_scale(p, -1, 20, true).ok())
                .map(|pixbuf|
                     Image {
                         path: path,
                         pixbuf: pixbuf,
                     }
                )
        }
    }
}

impl Drawable for Image {
    fn draw(&self, w: &mut Window, c: &Config, p: Position) -> Size {
        unsafe {
            println!("image at {:?}", p);
            cairo_move_to(w.context, p.x, p.y);
            let size = Size {
                width: self.pixbuf.get_width(),
                height: self.pixbuf.get_height(),
            };
            gdk_cairo_set_source_pixbuf(
                w.context, self.pixbuf.to_glib_none().0, p.x, p.y);
            cairo_rectangle(w.context, p.x, p.y,
                            self.pixbuf.get_width() as f64,
                            self.pixbuf.get_height() as f64);
            cairo_fill(w.context);
            size
        }
    }
}
