//! Here is a cool example with enums!
//!
//!```no_run
//! use eprompt::*;
//! use std::fmt::Display;
//!
//! #[derive(Debug)]
//! enum Choices {
//!     Opt1(i32),
//!     Opt2(&'static str),
//!     Opt3,
//! }
//!
//! impl Display for Choices {
//!     fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
//!         match *self {
//!             Self::Opt1(x) => write!(fmt, "{} of Option 1", x),
//!             Self::Opt2(x) => write!(fmt, "Option 2 is {}", x),
//!             Self::Opt3 => write!(fmt, "Mystery option 3"),
//!         }
//!     }
//! }
//!
//! fn main() -> Result<(), std::io::Error> {
//!     for choice in multi_select("Make a selection",
//!         &[Choices::Opt1(69), Choices::Opt2("A Brand new pony"), Choices::Opt3]
//!     )?.map(|x| x.1) {
//!         match choice {
//!             Choices::Opt1(x) => todo!("Do something with option 1"),
//!             Choices::Opt2(x) => todo!("Do something with option 2"),
//!     }
//!     
//!     Ok(())
//! }
//! ```

mod select;
pub use select::*;
mod fuzzy;
pub use fuzzy::*;
mod input;
pub use input::*;
