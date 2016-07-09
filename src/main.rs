extern crate cairo_sys as cairo;
extern crate pango;
extern crate pango_sys;
extern crate glib;
extern crate gobject_sys as gobject;
extern crate x11;

#[macro_use]
extern crate nom;

use std::io;
use std::io::BufRead;

use cairo::*;
mod pangocairo_gen;
use nom::IResult;

mod drawables;
mod window;
mod draw;
use draw::Color;
use window::Window;
use draw::Drawable;
use drawables::{Context, ContextBuilder};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 20;

named!(
    context( &str ) -> Box<Drawable>,
    map!(
        delimited!(tag_s!("{"), is_not_s!("}"), tag_s!("}")),
        |_| Box::new(ContextBuilder::empty())
    )
);


fn is_not_key(c: char) -> bool {
    c != '{'
}

named!(
    text( &str ) -> Box<Drawable>,
    map!(
        take_while_s!(is_not_key),
        |s| Box::new(String::from(s))
    )
);
// take_while_s!(is_not_key)
// take_until_s!("{")

named!(
    drawables( &str ) -> Vec<Box<Drawable> >,
    many0!(
        alt!(
            context |
            text
        )
    )
);

fn main() {
    let mut w = Window::new("title", WIDTH as u32, HEIGHT as u32);

    // println!("text: {:?}", text("(aaa){}"));
    // println!("context: {:?}", context("{fuck} aaaa"));

    // let thing: IResult<&str, Vec<Box<Drawable>>> = drawables("leleel {bla}");
    let thing = drawables("leleel {bla}");
    println!("the thing: {:?}", thing.is_done());


    /*
    // let string = many::<Vec<char>>(satisfy(|c: char| -> bool { c != '{' } ));
    // let string = many(any().skip(not_followed_by(char('{'))));
    // let mut context = between(char('{'), char('}'), many(any())).map(|_| ContextBuilder::empty());
    let mut context = between(
        char('{'),
        char('}'),
        many(any().skip(not_followed_by(char('}'))))
    ) ;
    //Parse spaces first and use the with method to only keep the result of the next parser
    let integer = spaces()
    //parse a string of digits into an i32
        .with(many1(digit()).map(|string: String| string.parse::<i32>().unwrap()));

    //Parse integers separated by commas, skipping whitespace
    let mut integer_list = sep_by(integer, spaces().skip(char(',')));

    //Call parse with the input to execute the parser
    let input = "1234, 45,78";
    let result: Result<(Vec<i32>, &str), ParseError<&str>> = integer_list.parse(input);

    // let input = "{string}";
    let result: Result<(String, &str), ParseError<&str>> = context.parse(input);
    match result {
        Ok((value, _remaining_input)) => println!("parsed: {:?}", value),
        Err(err) => println!("{}", err)
    }
     */


    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let root = Context::new("Terminus 10", Color::white(), 0.0);
        let mut base = ContextBuilder::empty();
        if let IResult::Done("", children) = drawables(&line.unwrap()) {
            println!("{:?}", children);
            base.children = children;
            w.clear();
            unsafe {
                cairo_move_to(w.context, 0.0, 0.0);
                base._draw(&mut w, &root);
            }
            w.flush();
        } else {
            println!("Failed parsing");
        };
    }
}
