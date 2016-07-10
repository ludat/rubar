use std::str::from_utf8;
use pango::FontDescription;

use super::text::quoted;

named!(pub font( &[u8] ) -> FontDescription,
       map!(
           map_res!(
               alt!(
                   quoted |
                   is_not!(" |{}")
               ),
               from_utf8
           ),
           FontDescription::from_string
       )
);

#[cfg(test)]
mod tests {
    use nom::IResult::{Done, Incomplete};
    use nom::Needed;

    use pango::FontDescription;

    use super::font;
    const EMPTY: &'static [u8] = &[];
    #[test] fn quoted() {
        assert_eq!(font(b"\"Sans\""),
                   Done(EMPTY, FontDescription::from_string("Sans")));
    }
    #[test] fn non_quoted() {
        assert_eq!(font(b"Sans"),
                   Done(EMPTY, FontDescription::from_string("Sans")));
    }
    #[test] fn unterminated_quote() {
        assert_eq!(font(b"\"Sans"),
                   Incomplete(Needed::Size(6))
        );
    }
}
