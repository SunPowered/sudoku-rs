use std::collections::{HashMap, HashSet};

use crate::{SudokuData, Index, Value, SudokuDataTree};

type PossibleValues = HashMap<Index, HashSet<Value>>;

pub struct SudokuSolver {
    pub solution: SudokuData,
    puzzle: SudokuDataTree,
    possibles: PossibleValues
}

impl SudokuSolver {
    pub fn new(puzzle_map: SudokuDataTree) -> SudokuSolver {
        let puzzle = puzzle_map.clone();
        let solution = SudokuData::from_map(puzzle_map);
        let possibles = solution.compute_possibles();
        return SudokuSolver {
            solution,
            puzzle,
            possibles
        }
    }

    fn update_solution(&mut self, index: Index, value: Value) {
        
        println!("Updating [{}] = {}", index, value);
        
        self.solution.set(index, value);

        for idx in self.solution.around_index(index) {
            if let Some(values) = self.possibles.get_mut(&idx) {                

                if values.contains(&value) {
                    if values.len() > 1 {
                        values.remove(&value);        
                    } else {
                        self.possibles.remove(&idx);
                    }
                }   
            }
        }
    }

    fn pass_over_possibles(&mut self) {
        let single_possibles = self.possibles.iter()
            .filter(|(_, vals)| vals.len() == 1)
            .map(|(idx, val)| (idx.clone(), val.iter().last().unwrap().clone()))
            .collect::<Vec<(Index, Value)>>();

        for (index, value) in single_possibles.iter() {
            self.update_solution(*index, *value);
        }
    }

    pub fn solve(&mut self) {
        println!("Attempting to solve puzzle");
        self.solution.print();
        let mut iter_count: usize = 0;

        let mut possible_size = self.possibles.len();

        self.pass_over_possibles();

        while self.possibles.len() != possible_size {
            iter_count += 1;
            possible_size = self.possibles.len();
            self.pass_over_possibles();
        }

        println!("After {} passes, there are {} positions remaining", iter_count, self.possibles.len());

        self.solution.print();

    }


}