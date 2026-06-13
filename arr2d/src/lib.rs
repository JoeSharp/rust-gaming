use crate::cell::Cell;
use std::hash::Hash;
pub mod cell;
mod copy;
mod core;
mod display;
mod parse;
mod partial_eq;
#[cfg(test)]
mod test_fixtures;
mod traverse;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidCharacter,
    NotEnoughLines,
    NotEnoughChars,
    InvalidValue,
}

#[derive(Debug, Hash, Eq)]
pub struct Arr2d<T> {
    contents: Vec<Vec<Cell<T>>>,
}
