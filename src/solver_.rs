use std::collections::{HashMap, HashSet};
use crate::{SudokuData, Index, Value, SudokuDataTree, Possibles, Indices};



pub struct SudokuSolver {
    pub solution: SudokuData,
    puzzle: SudokuDataTree,
    possibles: Possibles
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
        self.possibles.set(&index, &value);
        for idx in self.solution.around_index(index) {
            self.possibles.remove(idx, &value);
        }
    }

    fn remove_single_possibles(&mut self) {
        // low hanging fruit
        let single_possibles = self.possibles.cells.iter()
            .filter(|(_, vals)| vals.len() == 1)
            .map(|(idx, val)| (idx.clone(), val.iter().last().unwrap().clone()))
            .collect::<Vec<(Index, Value)>>();

        for (index, value) in single_possibles.iter() {
            self.update_solution(*index, *value);
        }
    }

    fn remove_single_values(&mut self) {
        // for each row, column, subsquare, check for any length-1 vectors from the possibleValues

        let mut single_values: HashMap<Index, Value> = HashMap::new();

        for idxs in Vec::from([&self.solution.indices.rows, &self.solution.indices.columns, &self.solution.indices.subsquares]) {
        
            for sub_idxs in idxs.iter() {
                // Get the indices from the possibles mappings
                let sub_set: HashSet<Index> = HashSet::from_iter(sub_idxs.clone());
                for (value, possible_idxs) in &self.possibles.values {
                    let sub_ = sub_set.clone();
                    let sub_possible_idxs = sub_.intersection(possible_idxs).collect::<Vec<&Index>>();
                    
                    if sub_possible_idxs.len() == 1 {
                        single_values.insert(*sub_possible_idxs[0], *value);
                    }
                }
            }
        }

        for (idx, value) in single_values {
            self.update_solution(idx, value);
        }
        
    }

    fn remove_subsquare_rowcols(&mut self) {
        // This is a hard one.  For each subsquare, check for any possibles that lie within a single subrow / subcol, update possibles accordingly

        let subsquare: Index = 3;

        let ss_idxs = &self.solution.indices.subsquares[subsquare];
        let supercol = subsquare % 3;
        let superrow = subsquare / 3;

        let rows: Vec<usize> = Vec::from_iter(superrow * 3 .. superrow * 3 +1); 
        let cols: Vec<usize> = Vec::from_iter(supercol * 3 .. supercol * 3 +1);

        let row_idxs: Vec<Indices> = rows.iter().map(|&i| self.solution.indices.rows[i].clone()).collect();
        let col_idxs: Vec<Indices> = cols.iter().map(|&i| self.solution.indices.columns[i].clone()).collect();
        

    }

    fn remove_from_subsquares_by_rows(&mut self) {
        for row in 0..9 {
            let row_idxs: Vec<Index> = self.solution.indices.rows[row].to_vec();

            let row_parts = vec!([&row_idxs[..3], &row_idxs[3..6], &row_idxs[6..9]]);

            for i in 0..3 {
                let rp_c = row_parts.clone();
                let idxs_in_ss = rp_c.remove(i);

                HashSet::from(idxs_in_ss);
                let mut hs: HashSet<Index> = HashSet::new();
                for item in rp_c {
                    for i in item {
                        hs.insert(*i);
                    }
                }

            }

            // This needs to be split into 3 3-len vectors
            
            //  If any value is possible only in one of the three sections, we can remove it from the other cell possibles in that subsquare
            

        }
    }

    pub fn solve(&mut self) {
        println!("Attempting to solve puzzle");
        self.solution.print();
        let mut iter_count: usize = 0;

        self.print_possibles();
        let mut possible_size = self.possibles.cells.len();

        self.remove_single_possibles();
        self.remove_single_values();

        while self.possibles.cells.len() != possible_size {
            iter_count += 1;
            possible_size = self.possibles.cells.len();
            self.remove_single_possibles();
            self.remove_single_values();
        }

        println!("After {} passes, there are {} positions remaining", iter_count, self.possibles.cells.len());
        self.print_possibles();
        self.solution.print();

        


    }

    fn print_possibles(&self) {
        let mut keys = self.possibles.cells.keys().collect::<Vec<&Index>>();
        keys.sort();

        for key in keys {
            println!("{}: {:?}", key, self.possibles.cells[key]);
        }

        let mut keys = self.possibles.values.keys().collect::<Vec<&Value>>();
        keys.sort();

        for key in keys {
            println!("{}: {:?}", key, self.possibles.values[key]);
        }

    }


}