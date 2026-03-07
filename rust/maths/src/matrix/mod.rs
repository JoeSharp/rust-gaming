mod approx_eq;
mod classification;
mod display;
mod main;
mod operations;

#[derive(PartialEq, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<f64>,
}

#[derive(Debug, PartialEq)]
pub enum MatrixError {
    IncorrectDataSize,
    IncompatibleDimensions,
    SquareMatrixRequired,
    InvalidIndex(usize, usize),
}
