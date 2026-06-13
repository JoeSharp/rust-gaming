use crate::Arr2d;
use std::fmt;
use std::fmt::Display;

impl<T> fmt::Display for Arr2d<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.contents {
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", val)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
