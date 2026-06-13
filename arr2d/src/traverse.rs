use crate::{Arr2d, Cell};
use std::collections::HashSet;

impl<T> Arr2d<T>
where
    T: PartialEq,
{
    pub fn get_perimeter(
        &self,
        row: usize,
        column: usize,
    ) -> Result<impl Iterator<Item = &Cell<T>>, &str> {
        let mut seen = HashSet::new();
        let start_cell = match self.get_cell(row, column) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        match self.flood_fill(row, column) {
            Ok(c) => Ok(c
                .flat_map(|cell| self.get_neighbours(cell.row(), cell.column()))
                .filter(|cell| !cell.value_eq(start_cell))
                .filter(move |cell| seen.insert(cell.id()))),
            Err(e) => Err(e),
        }
    }

    pub fn flood_fill(
        &self,
        row: usize,
        column: usize,
    ) -> Result<impl Iterator<Item = &Cell<T>>, &str> {
        let mut to_visit: Vec<&Cell<T>> = Vec::new();
        let mut ids_seen: HashSet<u32> = HashSet::new();
        let start_cell = match self.get_cell(row, column) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        to_visit.push(start_cell);
        ids_seen.insert(start_cell.id());

        Ok(std::iter::from_fn(move || match to_visit.pop() {
            Some(cell) => {
                self.get_neighbours(cell.row(), cell.column())
                    .filter(|c| ids_seen.insert(c.id()) && c.value_eq(start_cell))
                    .for_each(|c| to_visit.push(c));

                return Some(cell);
            }
            None => None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Arr2d;
    use super::Cell;
    use crate::test_fixtures::{
        Coordinate, ExpectedCell, TestBool, assert_cells, ex_cells_with_value,
    };
    use test_case::test_case;

    #[test_case((1, 1), vec![(1, 0, false), (0, 3, false), (2, 1, false), (1, 2, false)])]
    #[test_case((2, 1), vec![(2, 0, true), (2, 3, true), (1, 1, true), (1, 4, true), (0, 2, true)])]
    fn test_get_perimeter((row, column): Coordinate, expected: Vec<ExpectedCell>) {
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
        let result: Vec<&Cell<TestBool>> = match input.get_perimeter(row, column) {
            Ok(i) => i.collect(),
            _ => panic!("Could not flood fill"),
        };

        // Then
        assert_cells(&result, expected);
    }

    #[test_case((1, 1, true), vec![(1, 1), (0, 0), (0, 1), (0, 2)])]
    #[test_case((2, 1, false), vec![(2, 1), (2, 2), (1, 2), (1, 3), (0, 3), (0, 4)])]
    fn test_flood_fill((row, column, value): ExpectedCell, expected: Vec<Coordinate>) {
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
        let result: Vec<&Cell<TestBool>> = match input.flood_fill(row, column) {
            Ok(i) => i.collect(),
            _ => panic!("Could not flood fill"),
        };

        // Then
        let expected_cells = ex_cells_with_value(expected, value);
        assert_cells(&result, expected_cells);
    }
}
