use crate::{Arr2d, ParseError};

impl<T> Arr2d<T>
where
    T: Copy,
    T: TryFrom<char>,
{
    pub fn from_lines<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Arr2d<T>, ParseError> {
        let mut rows: Vec<Vec<T>> = Vec::new();

        for row in lines {
            let mut cells: Vec<T> = Vec::new();
            for cell in row.trim().chars() {
                match <T>::try_from(cell) {
                    Ok(v) => cells.push(v),
                    Err(_) => return Err(ParseError::InvalidCharacter),
                }
            }
            rows.push(cells);
        }

        Ok(Arr2d::from_contents(rows))
    }

    pub fn from_str(as_str: &str) -> Result<Arr2d<T>, ParseError> {
        Self::from_lines(
            as_str
                .split("\n")
                .map(|line| line.trim())
                .filter(|line| !line.is_empty()),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Arr2d;
    use crate::test_fixtures::TestBool;

    #[test]
    fn test_from_str() {
        // Given
        let expected: Arr2d<TestBool> = Arr2d::from_contents(vec![
            vec![
                TestBool::from(true),
                TestBool::from(true),
                TestBool::from(true),
                TestBool::from(false),
                TestBool::from(false),
            ],
            vec![
                TestBool::from(false),
                TestBool::from(true),
                TestBool::from(false),
                TestBool::from(false),
                TestBool::from(true),
            ],
            vec![
                TestBool::from(true),
                TestBool::from(false),
                TestBool::from(false),
                TestBool::from(true),
                TestBool::from(false),
            ],
        ]);

        // When
        let result: Arr2d<TestBool> = Arr2d::from_str(
            r#"
            yyynn
            nynny
            ynnyn
"#,
        )
        .expect("Arr2d should have parsed test input");

        // Then
        assert_eq!(result, expected);
    }
}
