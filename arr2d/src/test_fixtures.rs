use crate::{Cell, ParseError};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TestBool(bool);
pub type Coordinate = (usize, usize);
pub type ExpectedCell = (usize, usize, bool);

pub fn assert_cells(result: &Vec<&Cell<TestBool>>, expected: Vec<ExpectedCell>) {
    assert_eq!(
        expected.len(),
        result.len(),
        "Results should only contain expected cells.\nActual: {:#?}\nExpected: {:#?}",
        result,
        expected
    );
    for (ex_row, ex_column, value) in expected {
        let expected_cell = Cell::from(ex_row, ex_column, TestBool::from(value));
        assert!(
            result.contains(&&expected_cell),
            "result {result:?} does not contain {expected_cell:?}"
        );
    }
}

pub fn ex_cells_with_value(coordinates: Vec<Coordinate>, value: bool) -> Vec<ExpectedCell> {
    coordinates
        .into_iter()
        .map(|(row, column)| (row, column, value))
        .collect()
}

impl fmt::Display for TestBool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<bool> for TestBool {
    fn from(v: bool) -> TestBool {
        TestBool(v)
    }
}

impl TryFrom<char> for TestBool {
    type Error = ParseError;

    fn try_from(c: char) -> Result<TestBool, ParseError> {
        Ok(TestBool(c == 'y'))
    }
}
impl Into<char> for TestBool {
    fn into(self) -> char {
        if self.0 { 'y' } else { 'n' }
    }
}
