mod char;
mod colours;
mod writer;

#[doc(hidden)]
pub use writer::_print;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[char::ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
