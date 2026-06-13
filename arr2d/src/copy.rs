use crate::core::get_coordinates;
use crate::{Arr2d, Cell};

impl<T> Arr2d<T>
where
    T: Copy,
{
    pub fn expand(&self, rows: usize, columns: usize, filler: T) -> Arr2d<T> {
        let mut arr = Arr2d::with_size(rows, columns, filler);
        for row in 0..usize::min(rows, self.rows) {
            for col in 0..usize::min(columns, self.columns) {
                if let Ok(c) = self.get(row, col) {
                    arr.set(row, col, c.clone());
                }
            }
        }

        arr
    }

    pub fn from_2d_array(contents_2d: Vec<Vec<T>>) -> Arr2d<T> {
        let rows = contents_2d.len();
        let columns = contents_2d.first().map_or(0, |r| r.len());

        let mut id = 0;
        let mut contents = Vec::with_capacity(rows * columns);

        for (r, row) in contents_2d.into_iter().enumerate() {
            assert!(
                row.len() == columns,
                "All rows must have the same number of columns"
            );

            for (c, value) in row.into_iter().enumerate() {
                id += 1;
                contents.push(Cell::new(id, r, c, value));
            }
        }

        Arr2d {
            rows,
            columns,
            contents,
        }
    }

    pub fn from_contents(rows: usize, columns: usize, contents: Vec<T>) -> Arr2d<T> {
        let mut id = 0;

        Arr2d {
            rows,
            columns,
            contents: contents
                .iter()
                .enumerate()
                .map(|(index, &value)| {
                    id += 1;
                    let (row, column) = get_coordinates(index, columns);
                    Cell::new(id, row, column, value)
                })
                .collect(),
        }
    }
}

impl<T> Arr2d<T>
where
    T: Copy,
    T: Into<char>,
{
    pub fn to_str(&self) -> String {
        let mut as_str = String::new();

        as_str.push_str(&format!("Rows {}, Columns {}\n", self.rows, self.columns));
        for row in 0..self.rows {
            for col in 0..self.columns {
                if let Ok(value) = self.get(row, col) {
                    as_str.push((*value).into())
                }
            }
            as_str.push_str("\n");
        }

        as_str
    }
}

#[cfg(test)]
mod tests {
    use crate::Arr2d;
    use crate::test_fixtures::TestBool;

    #[test]
    fn test_expand() {
        // Given
        let a: Arr2d<TestBool> = Arr2d::new();
        let expected: Arr2d<TestBool> = Arr2d::from_2d_array(vec![
            vec![
                TestBool::from(false),
                TestBool::from(false),
                TestBool::from(false),
            ],
            vec![
                TestBool::from(false),
                TestBool::from(false),
                TestBool::from(false),
            ],
            vec![
                TestBool::from(false),
                TestBool::from(false),
                TestBool::from(false),
            ],
            vec![
                TestBool::from(false),
                TestBool::from(false),
                TestBool::from(false),
            ],
            vec![
                TestBool::from(false),
                TestBool::from(false),
                TestBool::from(false),
            ],
        ]);

        // When
        let result = a.expand(5, 3, TestBool::from(false));

        // Then
        assert_eq!(expected, result);
    }
}
