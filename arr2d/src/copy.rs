use crate::{Arr2d, Cell};

impl<T> Arr2d<T>
where
    T: Copy,
{
    pub fn expand(&self, width: usize, height: usize, filler: T) -> Arr2d<T> {
        let mut contents: Vec<Vec<T>> = self
            .contents
            .iter()
            .map(|v| v.iter().map(|c| c.value()).collect())
            .collect();

        for row in contents.iter_mut() {
            while row.len() < width {
                row.push(filler);
            }
            while row.len() > width {
                row.pop();
            }
        }

        while contents.len() < height {
            contents.push(vec![filler; width]);
        }
        while contents.len() > height {
            contents.pop();
        }

        Arr2d::from_contents(contents)
    }

    pub fn from_contents(contents: Vec<Vec<T>>) -> Arr2d<T> {
        let mut id = 0;

        Arr2d {
            contents: contents
                .iter()
                .enumerate()
                .map(|(row, row_c)| {
                    row_c
                        .iter()
                        .enumerate()
                        .map(|(column, &value)| {
                            id += 1;
                            Cell::new(id, row, column, value)
                        })
                        .collect()
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
        for row in &self.contents {
            for cell in row {
                as_str.push(cell.value().into());
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
        let expected: Arr2d<TestBool> = Arr2d::from_contents(vec![
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
        let result = a.expand(3, 5, TestBool::from(false));

        // Then
        assert_eq!(expected, result);
    }
}
