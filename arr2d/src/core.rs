use crate::Arr2d;
use crate::cell::Cell;

impl<T> Arr2d<T> {
    pub fn new() -> Arr2d<T> {
        Arr2d {
            contents: Vec::new(),
        }
    }

    pub fn rows(&self) -> usize {
        self.contents.len()
    }

    pub fn columns(&self, row: usize) -> usize {
        self.contents[row].len()
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        self.contents[row][col].value_ref()
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.contents[row][col].set_value(value);
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.contents.len(), self.contents[0].len())
    }

    pub fn with_size(rows: usize, columns: usize, default_value: T) -> Arr2d<T>
    where
        T: Copy,
    {
        let mut id = 0;
        Arr2d {
            contents: (0..rows)
                .map(|r| {
                    (0..columns)
                        .map(|c| {
                            id += 1;
                            Cell::new(id, r, c, default_value)
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Result<&Cell<T>, &str> {
        match &self.contents.get(row) {
            Some(r) => match r.get(column) {
                Some(c) => Ok(c),
                None => return Err("Invalid column index"),
            },
            None => return Err("Invalid row index"),
        }
    }

    pub fn all_cells(&self) -> impl Iterator<Item = &Cell<T>> {
        self.contents.iter().flat_map(|row| row.into_iter())
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
