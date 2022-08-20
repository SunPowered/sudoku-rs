use std::collections::HashMap;
use arrayvec::ArrayVec;

pub type Value = u8;
pub type SudokuDataCell = Option<Value>;
pub type SudokuDataTree = HashMap<usize, Value>;

pub type Subset = ArrayVec<SudokuDataCell, 9>;
pub type Dataset = ArrayVec<SudokuDataCell, 81>;
pub type Index = usize;
pub type Indices = ArrayVec<Index, 9>;

mod data;

pub use crate::data::SudokuData;
