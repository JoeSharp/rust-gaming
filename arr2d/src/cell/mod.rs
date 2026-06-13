mod core;
mod display;
mod partial_eq;

#[derive(Eq, Hash, Debug)]
pub struct Cell<T> {
    id: u32,
    row: usize,
    column: usize,
    value: T,
}
