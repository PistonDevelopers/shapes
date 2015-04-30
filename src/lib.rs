#![deny(missing_docs)]

//! Convenience structs for 2D shapes
extern crate graphics;

pub use self::line::Line;
pub use self::point::Point;
pub use self::rect::Rect;
pub use self::size::Size;

mod line;
mod point;
mod rect;
mod size;
