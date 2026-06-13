use crate::Arr2d;
use std::fmt;
use std::fmt::Display;

impl<T> fmt::Display for Arr2d<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.columns {
                if col > 0 {
                    write!(f, " ")?;
                }
                if let Ok(v) = self.get(row, col) {
                    write!(f, "{}", v)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
