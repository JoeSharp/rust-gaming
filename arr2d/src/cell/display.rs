use crate::Cell;
use std::fmt;
use std::fmt::Display;

impl<T> fmt::Display for Cell<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
