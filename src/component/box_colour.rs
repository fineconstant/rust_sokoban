use std::fmt::{Display, Formatter};
use core::fmt;

#[derive(PartialEq)]
pub enum BoxColour {
    Red,
    Blue,
}

impl Display for BoxColour {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            BoxColour::Red => "red",
            BoxColour::Blue => "blue",
        })
    }
}