use crate::Cell;

impl<T> Cell<T>
where
    T: Copy,
{
    pub fn value(&self) -> T {
        self.value
    }
}

impl<T> Cell<T>
where
    T: PartialEq,
{
    pub fn value_eq(&self, other: &Cell<T>) -> bool {
        self.value == other.value
    }
}

impl<T> Cell<T> {
    pub fn new(id: u32, row: usize, column: usize, value: T) -> Cell<T> {
        Cell {
            id,
            row,
            column,
            value,
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn row(&self) -> usize {
        self.row
    }
    pub fn column(&self) -> usize {
        self.column
    }
    pub fn value_ref(&self) -> &T {
        &self.value
    }
    pub fn set_value(&mut self, new_value: T) {
        self.value = new_value;
    }

    pub fn from(row: usize, column: usize, value: T) -> Cell<T> {
        Cell {
            id: 0,
            row,
            column,
            value,
        }
    }
}
