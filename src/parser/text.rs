use std::str::from_utf8;

named!(pub quoted( &[u8] ) -> &[u8],
       delimited!(tag!("\""), is_not!("\""), tag!("\""))
);

named!(pub text( &[u8] ) -> String,
       map!(
           map_res!(
               is_not!("{}"),
               from_utf8
           ),
           String::from
       )
);

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use nom::IResult::{Error, Done};
    use nom::Err::Position;
    use nom::ErrorKind;

    const EMPTY: &'static [u8] = &[];

    use super::text;
    #[test] fn empty_string() {
        assert_eq!(text(b""),
                   Done(EMPTY, "".to_string()));
    }
    #[test] fn only_one_char() {
        assert_eq!(text(b"a"),
                   Done(EMPTY, "a".to_string()));
    }
    #[test] fn string_with_a_space() {
        assert_eq!(text(b"bla bla"),
                   Done(EMPTY, "bla bla".to_string()));
    }
    #[test] fn string_followed_by_object() {
        assert_eq!(text(b"bla bla{ thing }"),
                   Done(&b"{ thing }"[..], "bla bla".to_string()));
    }
    #[test] fn open_cbracket_string_closing_cbracket() {
        assert_eq!(text(b"{ thing }"),
                   Error(Position(ErrorKind::IsNot, &b"{ thing }"[..])));
    }
    #[test] fn closing_curly_bracket_should_fail() {
        assert_eq!(text(b"}"),
                   Error(Position(ErrorKind::IsNot, &b"}"[..])));
    }
}
