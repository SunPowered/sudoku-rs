use std::collections::{HashMap, HashSet};
use arrayvec::ArrayVec;

type SudokuDataCell = Option<u8>;
type SudokuDataTree = HashMap<usize, u8>;

type Subset = ArrayVec<SudokuDataCell, 9>;
type Dataset = ArrayVec<SudokuDataCell, 81>;
type Index = usize;
type Indices = ArrayVec<Index, 9>;

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
    Error {index: u8, message: String},
    Valid,
    Complete
}


pub fn check_subset(subset: Subset) -> SubsetState {
    let values: Vec<u8> = subset.iter().filter_map(|x| *x).collect();
    let unique_values: HashSet<u8> = HashSet::from_iter(values);
    if unique_values.len() == subset.len() {
        return SubsetState::Complete;
    } else {
        // TODO: Check for errors (ie more than one non None value)
        return SubsetState::Valid
    }
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

}