use draw::Drawable;
use std::str::from_utf8;
use drawables::ContextBuilder;

mod color;

named!(context( &[u8] ) -> Box<Drawable>,
    map!(
        delimited!(tag!("{"), is_not!("}"), tag!("}")),
        |_| Box::new(ContextBuilder::empty())
    )
);

// named!(
//     context( &str ) -> Box<Drawable>,
//     map!(
//         chain!(
//             tag_s!("{"),
//             is_not_s!("}"),
//             tag_s!("}")),
//         |_| Box::new(ContextBuilder::empty())
//     )
// );


fn is_not_key(c: u8) -> bool {
    c != '{' as u8
}

named!(quoted( &[u8] ) -> &[u8],
    delimited!(tag!("\""), is_not!("\""), tag!("\""))
);

named!(text( &[u8] ) -> Box<Drawable>,
    map!(
        map_res!(
            take_while!(is_not_key),
            from_utf8
        ),
        |s| Box::new(String::from(s))
    )
);
// take_while_s!(is_not_key)
// take_until_s!("{")

named!(pub drawables( &[u8] ) -> Vec<Box<Drawable> >,
       many0!(
           alt!(
               context |
               text
           )
       )
);

