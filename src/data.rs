use std::collections::HashSet;

use crate::{Index, Subset, Dataset, SudokuDataTree, SudokuDataCell, Value, CachedIndices, PossibleCells, PossibleValues};




pub struct Possibles {
    pub cells: PossibleCells,
    pub values: PossibleValues
}

impl Possibles {
    pub fn remove(&mut self, &index: &Index, &value: &Value) {
        if let Some(i) = self.cells.get_mut(&index) {
            i.remove(&value);           
        }

        if let Some(i) = self.values.get_mut(&value) {
            i.remove(&index);           
        }
    }

    pub fn set(&mut self, &index: &Index, &value: &Value) {
        self.cells.remove(&index);
        if let Some(i) = self.values.get_mut(&value) {
            i.remove(&index);           
        }


    }

}

pub struct SudokuData {
    pub data: Dataset,
    pub indices: CachedIndices
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

    fn filter_subset_values(&self, subset: Subset) -> HashSet<Value> {
        let vec: Vec<Value> = subset.into_iter().filter(|i| match i {
            Some(_) => true,
            None => false
        }).map(|i| i.unwrap()).collect();
        HashSet::from_iter(vec)
    }

    fn row_values(&self, row_index: Index) -> HashSet<Value> {
        self.filter_subset_values(self.row(row_index))
    }

    pub fn column(&self, column_index: Index) -> Subset {
        self.indices.columns[column_index].iter().map(|i| self.data[*i]).collect()
    }
    
    fn column_values(&self, column_index: Index) -> HashSet<Value> {
        self.filter_subset_values(self.column(column_index))
    }

    pub fn subsquare(&self, subsquare_index: Index) -> Subset {
        self.indices.subsquares[subsquare_index].iter().map(|i| self.data[*i]).collect()
    }

    fn subsquare_values(&self, subsquare_index: Index) -> HashSet<Value> {
        self.filter_subset_values(self.subsquare(subsquare_index))
    }

    pub fn around_index(&self, index: Index) -> Vec<&Index>{
        let (row, column, subsquare) = self.position_from_index(index);

        let not_the_index = |i: &&Index| { *i != &index };
        let mut row_indices = self.indices.rows[row].iter().collect::<Vec<&Index>>();
        let mut column_indices = self.indices.columns[column].iter().filter(not_the_index).collect::<Vec<&Index>>();
        let mut subsquare_indices = self.indices.subsquares[subsquare].iter().filter(not_the_index).collect::<Vec<&Index>>();
        
        row_indices.append(&mut column_indices);
        row_indices.append(&mut subsquare_indices);
        row_indices

    }

    pub fn set(&mut self, index: usize, value: u8) {
        if index >= 81 {
            panic!("Index out of bounds [0, 81)")
        }
        self.data[index] = Some(value);
        
    }

    fn position_from_index(&self, index: Index) -> (Index, Index, Index){
        let row = index / 9;
        let column=  index % 9;
        let subsquare = 3 * (row / 3) + (column / 3);
        (row, column, subsquare)

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

    // fn check(&self) -> Vec<SubsetCheckedState> {
    //     let row_checks: Vec<SubsetCheckedState> = (0..9).map(|i| check_subset(self.row(i))).collect();
    //     let column_checks = (0..9).map(|i| check_subset(self.column(i))).collect();
    //     let subsquare_checks = (0..9).map(|i| check_subset(self.subsquare(i))).collect();
        
    //     let f = |arr: &Vec<SubsetCheckedState>| {
    //         if let Some(error) = arr.iter().filter(|i| i.is_error()).next() {
    //             return *error
    //         }
    //         if arr.iter().all(|i| i.is_complete()) {return SubsetCheckedState::Complete}
    //         return SubsetCheckedState::Valid
    //     };

    //     return vec!(row_checks, column_checks, subsquare_checks).iter().map(|i| f(i)).collect();


    // }

    fn possibles_for_index(&self, index: Index) -> HashSet<Value> {
        let (row, column, subsquare) = self.position_from_index(index);
        let values_set = HashSet::from_iter((1..10).into_iter());
        let mut bad_values: HashSet<Value> = HashSet::new();
        bad_values.extend(self.row_values(row));
        bad_values.extend(self.column_values(column));
        bad_values.extend(self.subsquare_values(subsquare));

        values_set.difference(&bad_values).cloned().collect()
    }

    fn compute_possible_cells(&self) -> PossibleCells {


        let mut possibles = PossibleCells::new();

        for (idx, value) in self.data.iter().enumerate() {
            match value {
                None => {
                    possibles.insert(idx, self.possibles_for_index(idx));
                },
                _ => {}
            }
        }
        possibles
    }

    pub fn compute_possibles(&self) -> Possibles {
        let possible_cells = self.compute_possible_cells();

        let mut possible_values = PossibleValues::new();

        for i in 1..10 {
            possible_values.insert(i, HashSet::new());
        }

        for (idx, values) in possible_cells.iter() {
            for value in values {
                if let Some(set) = possible_values.get_mut(value) {
                    set.insert(*idx);
                }

            }
        }

        Possibles {cells: possible_cells, values: possible_values}
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

// #[test]
// fn check_subset_complete() {
//     let subset = Subset::from([Some(4),Some(6),Some(8),Some(2),Some(3),Some(1),Some(9),Some(7),Some(5)]);
//     let state = check_subset(subset);
//     assert_eq!(state, SubsetCheckedState::Complete);
//     assert!(state.is_complete());
// }

// #[test]
// fn check_subset_error() {
//     let subset = Subset::from([Some(1),Some(2),Some(3),Some(2),Some(5),Some(6),Some(7),Some(8),Some(9)]);
//     let state = check_subset(subset);
//     assert!(state.is_error());
//     match state {
//        SubsetCheckedState::Error(_) => {},
//         _ => {panic!("Expected Error");}
//    }
// }
}