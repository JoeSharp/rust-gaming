use crate::cell::Cell;

impl<T> PartialEq for Cell<T>
where
    T: PartialEq,
{
    fn eq(&self, b: &Cell<T>) -> bool {
        self.row == b.row && self.column == b.column && self.value == b.value
    }
}
