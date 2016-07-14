use std::str::{FromStr, from_utf8};

use nom::digit;
// use draw::{Position, Size};

named!(geometry( &[ u8 ]) -> (Option<f64>, Option<f64>, Option<i32>, Option<i32>),
       chain!(
           x: opt!(
               map!(
                   number,
                   |n| n as f64
               )
           ) ~
           tag!("x") ~
           y: opt!(
               map!(
                   number,
                   |n| n as f64
               )
           ) ~
           w: signed_num ~
           h: signed_num,
           || (x, y, w, h)
       )
);

named!(signed_num( &[u8] ) -> Option<i32>,
       chain!(
           sign: alt!(
               value!(1,  tag!("+")) |
               value!(-1, tag!("-"))
           ) ~
           num: opt!(number),
           || num.map(|num| sign * num as i32)
       )
);

named!(number( &[u8] ) -> u32,
       map_res!(
           map_res!(
               digit,
               from_utf8
           ),
           FromStr::from_str
       )
);

#[cfg(test)]
mod tests {
    const EMPTY: &'static [u8] = &[];

    use nom::IResult::Done;

    mod geometry {
        use nom::IResult::Done;

        use super::EMPTY;
        use super::super::geometry;
        #[test] fn four_ones() {
            assert_eq!(
                geometry(b"1x1+1+1"),
                Done(EMPTY, (Some(1.0), Some(1.0), Some(1), Some(1))));
        }
        #[test] fn missing_x() {
            assert_eq!(
                geometry(b"x1+1+1"),
                Done(EMPTY, (None, Some(1.0), Some(1), Some(1))));
        }
        #[test] fn missing_width() {
            assert_eq!(
                geometry(b"1x1++1"),
                Done(EMPTY, (Some(1.0), Some(1.0), None, Some(1))));
        }
        #[test] fn missing_width_and_heigh() {
            assert_eq!(
                geometry(b"1x1++"),
                Done(EMPTY, (
                    Some(1.0),
                    Some(1.0),
                    None,
                    None)));
        }
        #[test] fn common_example() {
            assert_eq!(
                geometry(b"x20++"),
                Done(EMPTY, (
                    None,
                    Some(20.0),
                    None,
                    None
                )));
        }
        #[test] fn big_numers() {
            assert_eq!(
                geometry(b"1000x200+10+"),
                Done(EMPTY, (Some(1000.0), Some(200.0), Some(10), None)));
        }
    }
    #[test] fn plus_one() {
        use super::signed_num;
        assert_eq!(
            signed_num(b"+1"),
            Done(EMPTY, Some(1)));
    }
    #[test] fn number() {
        use super::number;
        assert_eq!(
            number(b"1"),
            Done(EMPTY, 1));
    }
}
