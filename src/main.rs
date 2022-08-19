
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::collections::HashMap;
use std::env;

use sudoku_rs::SudokuData;




fn read_lines(path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path).expect("Unable to open file");
    Ok(io::BufReader::new(file).lines())
}

fn read_puzzle_file(filepath: &str) -> Result<HashMap<usize, u8>, Error> {
    let mut data_map: HashMap<usize, u8>  = HashMap::new();

    if let Ok(lines) = read_lines(filepath) {
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
    }

    return Ok(data_map)
}

fn main() {
    println!("Sudoku Solver");

    let filepath = env::args().nth(1).expect("No filepath given");

    let data_map = read_puzzle_file(&filepath).expect("Could not read provided file");


    let puzzle_data = SudokuData::from_map(data_map);

    println!("Read puzzle_data");
    println!("First row: {:?}", puzzle_data.row(0));
    puzzle_data.print();
}
