mod text;
mod context;
pub use drawables::context::{
    Config,
    Context
};
// use draw::{Drawable, Size};
// use window::Window;

// pub enum Draw<'a> {
//     Text(&'a str),
//     Context(Context<'a>),
// }

// impl<'a> Drawable for Draw<'a> {
//     unsafe fn _draw(&self, w: &mut Window, context: &Context) -> Size {
//         match *self {
//             Draw::Text(ref s) => s._draw(w, context),
//             Draw::Context(ref c) => c._draw(w, context)
//         }
//     }
// }
