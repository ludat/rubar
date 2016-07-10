use super::font::font;
use super::color::color;
use super::text::text;
use super::image::image;
use pango::FontDescription;

use nom::{space, be_f64};

use drawables::{Context};
use draw::{Draw, Color};

named!(pub draw( &[u8] ) -> Vec<Draw>,
       many0!(
           alt!(
               map!(
                   image,
                   Draw::Image
               ) |
               map!(
                   context,
                   Draw::Context
               ) |
               map!(
                   text,
                   Draw::Text
               )
           )
       )
);

named!(pub context( &[u8] ) -> Context,
       chain!(
           mut ctx: value!(Context::empty(), tag!("{")) ~
           many0!(
               alt!(
                   map!(alpha_prop, |a| { ctx.alpha = Some(a) }) |
                   map!(color_prop, |c| { ctx.color = Some(c) }) |
                   map!(font_prop,  |f| { ctx.font  = Some(f) })
               )
           )  ~
           space? ~
           tag!("|") ~
           map!(draw, |children| { ctx.children = children }) ~
           tag!("}"),
           || ctx
       )
);

named!(font_prop( &[u8] ) -> FontDescription,
       chain!(
           tag!("font=") ~
           f: font ~
           space?,
           || f
       )
);

named!(color_prop( &[u8] ) -> Color,
       chain!(
           tag!("color=") ~
           c: color ~
           space?,
           || c
       )
);

named!(alpha_prop( &[u8] ) -> f64,
       chain!(
           tag!("alpha=") ~
           a: be_f64 ~
           space?,
           || a
       )
);

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    const EMPTY: &'static [u8] = &[];

    mod context {
        use nom::IResult::{Error, Done};
        use nom::Err::Position;
        use nom::ErrorKind;

        use drawables::Context;
        use draw::Color;
        use pango::FontDescription as Font;
        use draw::Draw;

        use super::EMPTY;
        use super::super::context;

        #[test] fn empty() {
            assert_eq!(
                context(b"{|}"),
                Done(EMPTY, Context::empty()));
        }
        #[test] fn only_color() {
            assert_eq!(
                context(b"{color=black|}"),
                Done(
                    EMPTY,
                    Context::empty().color(Color::black())
                )
            );
        }
        #[test] fn color_and_font() {
            assert_eq!(
                context(b"{font=\"Sans\" color=black|}"),
                Done(
                    EMPTY,
                    Context::empty()
                        .color(Color::black())
                        .font(Font::from_string("Sans"))
                )
            );
        }
        #[test] fn color_font_and_content() {
            assert_eq!(
                context(b"{font=\"Sans\" color=blue|this string}"),
                Done(
                    EMPTY,
                    Context::empty()
                        .font(Font::from_string("Sans"))
                        .color(Color::blue())
                        .push(Draw::Text("this string".to_string()))
                )
            );
        }
        #[test] fn hex_color_and_context() {
            assert_eq!(
                context(b"{color=#0000ff|this string}"),
                Done(
                    EMPTY,
                    Context::empty()
                        .color(Color::blue())
                        .push(Draw::Text("this string".to_string()))
                )
            );
        }
    }
}
