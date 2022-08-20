use arrayvec::ArrayVec;
use std::collections::HashSet;

use crate::{Index, Indices, Subset, Dataset, SudokuDataTree, SudokuDataCell, Value};

fn row_indices(row_index: Index) -> Indices {
    (0..9).map(|i| 9 * row_index + i).collect()
}

fn column_indices(column_index: Index) -> Indices {
    (0..9).map(|i| column_index + 9 * i).collect()
}

fn subsquare_indices(subsquare_index: Index) -> Indices {

    let mut subsquare = ArrayVec::from([0; 9]);
    let row = subsquare_index / 3;
    let column = subsquare_index % 3;
    let start = 27 * row + 3 * column;
    for i in 0..3 {
        for j in 0..3 {
            subsquare[3*i+j] = start + 9 * i + j;
        }
    }
    subsquare
}

fn generate_indices(f: fn(usize) -> Indices) -> ArrayVec<Indices, 9> {
    (0..9).map(|i| f(i)).collect()
}

struct CachedIndices {
    rows: ArrayVec<Indices, 9>,
    columns: ArrayVec<Indices, 9>,
    subsquares: ArrayVec<Indices, 9>
}

impl CachedIndices {
    fn new() -> CachedIndices{
        return CachedIndices {
            rows: generate_indices(row_indices),
            columns: generate_indices(column_indices),
            subsquares: generate_indices(subsquare_indices)
        }
    }
}

pub enum SubsetState {
    Error {index: Index, message: String},
    Valid,
    Complete
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SubsetCheckedState {
    Complete,
    Valid,
    Error(SudokuDataCell)
}

impl SubsetCheckedState {
    fn is_error(&self) -> bool {
        match self {
            SubsetCheckedState::Error(_) => {
                true
            }
            _ => {
                false
            }
        }
    }

    fn is_complete(&self) -> bool {
        match self {
            SubsetCheckedState::Complete => {
                true
            }
            _ => {
                false
            }
        }
    }
}


pub fn check_subset(data: Subset) -> SubsetCheckedState {
    
    let values: Vec<Value> = data.iter().filter(|i| !i.is_none()).map(|i| i.unwrap()).collect();
    let hash_set: HashSet<u8> = HashSet::from_iter(values.clone());

    if values.len() != hash_set.len() {
        // Duplicate values
        return SubsetCheckedState::Error(None)
    } 
    if hash_set.len() == 9 {
        // Complete
        return SubsetCheckedState::Complete
    }     
    // Valid, yet incomplete
    return SubsetCheckedState::Valid    

}

pub struct SudokuData {
    data: Dataset,
    indices: CachedIndices
}

impl SudokuData {
    pub fn from_map(data_tree: SudokuDataTree) -> SudokuData {
        let mut data = Dataset::from([None;81]);

        for (index, value) in data_tree {
            data[index] = Some(value)
        }

        return SudokuData::new(data)
    }

    pub fn new(data: Dataset) -> SudokuData {
        return SudokuData {data, indices: CachedIndices::new()}
    }

    pub fn row(&self, row_index: Index) -> Subset{
        self.indices.rows[row_index].iter().map(|i| self.data[*i]).collect()
    }

    pub fn column(&self, column_index: Index) -> Subset {
        self.indices.columns[column_index].iter().map(|i| self.data[*i]).collect()
    }

    pub fn subsquare(&self, subsquare_index: Index) -> Subset {
        self.indices.subsquares[subsquare_index].iter().map(|i| self.data[*i]).collect()
    }

    pub fn set(&mut self, index: usize, value: u8) {
        if index >= 81 {
            panic!("Index out of bounds [0, 81)")
        }
        self.data[index] = Some(value);
        
    }

    pub fn print(&self) {
        let horizontal_line ="+ - + - + - + - + - + - + - + - + - +";
        
        for row_index in 0..9 {
            if (row_index % 3 == 0) && (row_index != 0) {
                println!("{}", horizontal_line);
            }
            self.print_line(row_index)
        }
        // println!("{}", horizontal_line);
    }

    fn print_line(&self, row_index: Index) {
        let no_value = " ";
        let row_vals = self.row(row_index);

        let row = row_vals.chunks(3)
            .map(|chunk| 
                chunk.iter().map(|item| 
                    match item {
                        Some(val) => val.to_string(),
                        None => no_value.into()
                    }).collect::<Vec<String>>().join("   ")
        ).collect::<Vec<String>>().join(" ¦ ");
        println!("  {} ", row);

    }

    fn check(&self) -> Vec<SubsetCheckedState> {
        let row_checks: Vec<SubsetCheckedState> = (0..9).map(|i| check_subset(self.row(i))).collect();
        let column_checks = (0..9).map(|i| check_subset(self.column(i))).collect();
        let subsquare_checks = (0..9).map(|i| check_subset(self.subsquare(i))).collect();
        
        let f = |arr: &Vec<SubsetCheckedState>| {
            if let Some(error) = arr.iter().filter(|i| i.is_error()).next() {
                return *error
            }
            if arr.iter().all(|i| i.is_complete()) {return SubsetCheckedState::Complete}
            return SubsetCheckedState::Valid
        };

        return vec!(row_checks, column_checks, subsquare_checks).iter().map(|i| f(i)).collect();


    }

    
}


#[cfg(test)]
mod tests {
    use super::*;
#[test]
fn test_from_data() {
    let data = Dataset::from([
        None, Some(8), None, Some(1), None, None, None, Some(7), None,
        Some(1), None, None, Some(4), None, None, None, None, None,
        None, None, None, None, Some(6), None, None, None, Some(9),
        None, Some(8), None, Some(1), None, None, None, Some(7), None,
        Some(1), None, None, Some(4), None, None, None, None, None,
        None, None, None, None, Some(6), None, None, None, Some(9),
        None, Some(8), None, Some(1), None, None, None, Some(7), None,
        Some(1), None, None, Some(4), None, None, None, None, None,
        None, None, None, None, Some(6), None, None, None, Some(9)
        ]);

    let sudoku_data = SudokuData::new(data);

    assert_eq!(
        sudoku_data.row(3), 
        Subset::from([None, Some(8), None, Some(1), None, None, None, Some(7), None])
    );
}

#[test]
fn test_from_hashmap() {
    let data_tree = SudokuDataTree::from([
        (3, 7),
        (8, 1),
        (9, 2),
        (12, 3),
        (28, 5),
        (29, 7),
        (80, 9)
    ]);
    
    let sudoku_data = SudokuData::from_map(data_tree);
    assert_eq!(sudoku_data.row(1),
        Subset::from([Some(2), None, None, Some(3), None, None, None, None, None])
    );

}

#[test]
fn test_row_indices() {
    let row_idxs = row_indices(4);
    assert_eq!(row_idxs, ArrayVec::from([36, 37, 38, 39, 40, 41, 42, 43, 44]));
}

#[test]
fn test_column_indices() {
    let col_idxs = column_indices(3);
    assert_eq!(col_idxs, ArrayVec::from([3, 12, 21, 30, 39, 48, 57, 66, 75]));
}

#[test]
fn test_subsquare_indices() {
    let subsquare_idxs = subsquare_indices(2);
    assert_eq!(subsquare_idxs, ArrayVec::from([6, 7, 8, 15, 16, 17, 24, 25, 26]));
}

#[test]
fn check_subset_complete() {
    let subset = Subset::from([Some(4),Some(6),Some(8),Some(2),Some(3),Some(1),Some(9),Some(7),Some(5)]);
    let state = check_subset(subset);
    assert_eq!(state, SubsetCheckedState::Complete);
    assert!(state.is_complete());
}

#[test]
fn check_subset_error() {
    let subset = Subset::from([Some(1),Some(2),Some(3),Some(2),Some(5),Some(6),Some(7),Some(8),Some(9)]);
    let state = check_subset(subset);
    assert!(state.is_error());
    match state {
       SubsetCheckedState::Error(_) => {},
        _ => {panic!("Expected Error");}
   }
}
}