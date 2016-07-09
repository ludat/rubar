use draw::{Drawable, Color};
use std::str::from_utf8;
use drawables::ContextBuilder;

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

named!(hexcolor( &[u8] ) -> Color,
       chain!(
           tag!("#") ~
           r: color_value ~
           g: color_value ~
           b: color_value,
           || Color::new(r/255.0, g/255.0, b/255.0)
       )
);

fn byte_to_hex(byte: u8) -> Option<u8> {
    match byte as char {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'a' | 'A' => Some(10),
        'b' | 'B' => Some(11),
        'c' | 'C' => Some(12),
        'd' | 'D' => Some(13),
        'e' | 'E' => Some(14),
        'f' | 'F' => Some(15),
        _ => None,
    }
}

fn bytes_to_hex(bytes: &[u8]) -> Option<f64> {
    bytes.iter().fold(Some(0.0), |acc, &n| {
        acc.and_then(|acc| {
            byte_to_hex(n).map(|n| acc * 16.0 + n as f64)
        })
    })
}

#[test]
fn test_hexcolor() {
    use nom::IResult::{Error, Done};
    use nom::Err::Position;
    use nom::ErrorKind;
    let empty = &b""[..];
    assert_eq!(hexcolor(b"#ffffff"),
               Done(empty, Color::new(1.0,1.0,1.0)));
    assert_eq!(hexcolor(b"#000000"),
               Done(empty, Color::new(0.0,0.0,0.0)));
    assert_eq!(hexcolor(b"#ffffff"),
               Done(empty, Color::new(1.0,1.0,1.0)));
    assert_eq!(hexcolor(b"#000000"),
               Done(empty, Color::new(0.0,0.0,0.0)));
    assert_eq!(hexcolor(b"#00000j"),
               Error(Position(ErrorKind::MapOpt, &b"0j"[..])));
    assert_eq!(hexcolor(b"jjjjjjj"),
               Error(Position(ErrorKind::Tag, &b"jjjjjjj"[..])));

}

named!(color_value( &[u8] ) -> f64,
       map_opt!(
           take!(2),
           bytes_to_hex
       )
);
