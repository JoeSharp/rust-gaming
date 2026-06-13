use crate::Arr2d;
use crate::cell::Cell;

pub fn get_coordinates(index: usize, columns: usize) -> (usize, usize) {
    (index / columns, index % columns)
}

impl<T> Arr2d<T> {
    pub fn new() -> Arr2d<T> {
        Arr2d {
            rows: 0,
            columns: 0,
            contents: Vec::new(),
        }
    }

    pub fn rows(&self) -> usize {
        self.contents.len()
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn get(&self, row: usize, col: usize) -> Result<&T, &str> {
        match self.get_index(row, col) {
            Ok(index) => match self.contents.get(index) {
                Some(cell) => Ok(cell.value_ref()),
                None => Err("Invalid index"),
            },
            Err(_) => Err("Invalid index"),
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if let Ok(index) = self.get_index(row, col) {
            if let Some(cell) = self.contents.get_mut(index) {
                cell.set_value(value);
            }
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }

    fn get_index(&self, row: usize, col: usize) -> Result<usize, ()> {
        if row >= self.rows {
            return Err(());
        }
        if col >= self.columns {
            return Err(());
        }
        Ok(col + row * self.columns)
    }

    pub fn with_size(rows: usize, columns: usize, default_value: T) -> Arr2d<T>
    where
        T: Copy,
    {
        let mut id = 0;
        let mut contents = Vec::with_capacity(rows * columns);

        for r in 0..rows {
            for c in 0..columns {
                id += 1;
                contents.push(Cell::new(id, r, c, default_value));
            }
        }

        Arr2d {
            rows,
            columns,
            contents,
        }
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Result<&Cell<T>, &str> {
        match self.get_index(row, column) {
            Ok(index) => match &self.contents.get(index) {
                Some(c) => Ok(c),
                None => return Err("Invalid index"),
            },
            Err(_) => return Err("Invalid index"),
        }
    }

    pub fn all_cells(&self) -> impl Iterator<Item = &Cell<T>> {
        self.contents.iter()
    }

    pub fn get_neighbours(&self, row: usize, column: usize) -> impl Iterator<Item = &Cell<T>> {
        [
            (Some(row), column.checked_sub(1)),
            (Some(row), column.checked_add(1)),
            (row.checked_sub(1), Some(column)),
            (row.checked_add(1), Some(column)),
        ]
        .into_iter()
        .filter_map(|(r, c)| {
            if let (Some(r), Some(c)) = (r, c) {
                println!("Checking Cell {}, {}", r, c);
                self.get_cell(r, c).ok()
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Arr2d;
    use crate::cell::Cell;
    use crate::test_fixtures::{Coordinate, ExpectedCell, TestBool, assert_cells};
    use test_case::test_case;

    #[test_case((0, 0), vec![(0, 1, true), (1, 0, false)] )]
    #[test_case((1, 1), vec![(0, 1, true), (2, 1, false), (1, 0, false), (1, 2, false)] )]
    #[test_case((2, 2), vec![(2, 1, false), (1, 2, false), (2, 3, true)] )]
    #[test_case((2, 4), vec![(2, 3, true), (1, 4, true)] )]
    #[test_case((0, 4), vec![(0, 3, false), (1, 4, true)] )]
    fn test_get_neighbours((row, column): Coordinate, expected: Vec<ExpectedCell>) {
        // Given
        let input: Arr2d<TestBool> = Arr2d::from_str(
            r#"
            yyynn
            nynny
            ynnyn
"#,
        )
        .expect("Arr2d should have parsed test input");

        // When
        let result: Vec<&Cell<TestBool>> = input.get_neighbours(row, column).collect();

        // Then
        assert_cells(&result, expected);
    }
}
