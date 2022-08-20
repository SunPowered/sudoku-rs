use crate::{SudokuData, Index, Value, SudokuDataTree};

type PossibleValues = HashMap<Index, Vec<Values>>;

struct SudokuSolver {
    solution: SudokuData,
    puzzle: SudokuDataTree,
    possibles: PossibleValues
}

impl SudokuSolver {
    fn new(puzzle_map: SudokuDataTree) -> SudokuSolver {
        let solution = SudokuData::from_map(puzzle_map);
        return SudokuSolver {
            solution,
            puzzle: puzzle_map,
            possibles: HashMap<Index, Vec<Values>>::new()
        }
    }
}