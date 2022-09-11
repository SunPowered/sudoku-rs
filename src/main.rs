
use std::env;

use sudoku_rs::{
    Solver,
    utils::{
        parse_from_file,
        print_puzzle
    }
};



fn main() {
    println!("Sudoku Solver");

    let filepath = env::args().nth(1).expect("No filepath given");

    //let data_map = read_puzzle_file(&filepath).expect("Could not read provided file");
    let initial_data = parse_from_file(&filepath).expect("Error reading puzzle");

    let mut solver = Solver::new(initial_data);
    // solver.solution.print();
    print_puzzle(&solver.data());
    solver.solve();
    print_puzzle(&solver.data());


}
