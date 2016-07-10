use nom::space;
use std::str::from_utf8;
use std::path::{Path, PathBuf};
use drawables::Image;


named!(pub image( &[u8] ) -> Image,
       map_opt!(
           chain!(
               tag!("{image") ~
               space? ~
               p: path ~
               tag!("}"),
               || p
           ),
           Image::new
       )
);

named!(path( &[u8] ) -> PathBuf,
       map!(
           map_res!(
               is_not!("}"),
               from_utf8
           ),
           |s| Path::new(s).to_path_buf()
       )
);

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    const EMPTY: &'static [u8] = &[];

    mod image {
        use nom::IResult::{Error, Done};
        use nom::Err::Position;
        use nom::ErrorKind;

        use drawables::Context;
        use draw::Color;
        use pango::FontDescription as Font;
        use draw::Draw;

        use super::EMPTY;

        #[test] fn empty() {
        }
    }
}
