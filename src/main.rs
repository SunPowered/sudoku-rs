
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::env;

use sudoku_rs::{SudokuDataTree, SudokuSolver};



fn read_puzzle_file(filepath: &str) -> Result<SudokuDataTree, Error> {
    let mut data_map: SudokuDataTree  = SudokuDataTree::new();

    let file = File::open(filepath).expect("Unable to open file");

    let lines = io::BufReader::new(file).lines();
    
    for line in lines {
        match line {
            Ok(buf) => {
                let split_strings: Vec<&str> = buf.split(",").collect();
                let key = split_strings[0].parse::<usize>().unwrap();
                let value = split_strings[1].parse::<u8>().unwrap();
                data_map.insert(key, value);
            },
            Err(e) => {
                println!("Error parsing line {:?}", e);
            }
        }         
    }
    

    return Ok(data_map)
}

fn main() {
    println!("Sudoku Solver");

    let filepath = env::args().nth(1).expect("No filepath given");

    let data_map = read_puzzle_file(&filepath).expect("Could not read provided file");

    let mut solver = SudokuSolver::new(data_map);
    // solver.solution.print();
    solver.solve();


}
