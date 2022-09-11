pub type Index = usize;
pub type Value = u8;
pub type DataCell = Option<Value>;
pub type Data = [DataCell;81];

#[derive(Debug)]
pub struct Position {
    row: Index,
    column: Index,
    subsquare: Index
}


mod indices;
mod possibles;
mod solver;
pub mod utils;

pub use crate::indices::Indices;
pub use crate::possibles::Possibles;
pub use crate::solver::Solver;