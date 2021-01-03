use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    None = 0,   // Empty
    Cyan = 1,   // I
    Yellow = 2, // O
    Purple = 3, // T
    Green = 4,  // S
    Red = 5,    // Z
    Blue = 6,   // J
    Orange = 7, // L
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
