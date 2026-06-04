use super::Matrix;
use std::fmt;
use std::fmt::Display;

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.rows {
            write!(f, "[")?;

            for c in 0..self.columns {
                match self.get(r, c) {
                    Ok(v) => write!(f, "{}", v),
                    _ => return Err(fmt::Error),
                }?;
                if c + 1 != self.columns {
                    write!(f, " ")?
                }
            }

            write!(f, "]")?;
            if r + 1 != self.rows {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
