//  Playing with custom iterators
//
//
use std::iter::{successors, FromIterator};
use std::collections::{HashMap, HashSet};

type Index = usize;
type Value = u8;
type Data = [Option<Value>; 81];

struct Position {
    row: Index,
    column: Index,
    subsquare: Index
}

struct Indices {
}

impl Indices {
    fn row(row: Index) -> Vec<Index>{
        successors(Some(9  * row), |i| Some(i+1)).take(9).collect()
    }

    fn column(column: Index) -> Vec<Index> {
        successors(Some(column), |i| Some(i+9)).take(9).collect()
    }

    fn subsquare(subsquare: Index) -> Vec<Index> {
        let start = 3 * (subsquare % 3) + 27 * (subsquare / 3);

        successors(Some(start), |i| Some(i+1)).take(3).chain(
            successors(Some(start + 9), |i| Some(i+1)).take(3).chain(
                successors(Some(start + 18), |i| Some(i+1)).take(3)
            )
        ).collect()
    }

    fn around(idx: usize) -> Vec<Index> {
        let position = Indices::position_from_index(idx);
        Indices::row(position.row).iter().cloned().chain(
            Indices::column(position.column).iter().cloned().chain(
                Indices::subsquare(position.subsquare).iter().cloned()
            )
        ).filter(|&i| i != idx).collect()
    }

    fn position_from_index(idx: Index) -> Position {
        let row = idx / 9;
        let column = idx % 9;
        let subsquare = 3 * (row / 3) + column / 3;   
        Position {row, column, subsquare}        
    }

}

#[derive(Debug)]
struct Possibles {
    by_cells: HashMap<Index, HashSet<Value>>,
    by_values: HashMap<Value, HashSet<Index>>
}

impl Possibles {
    fn from_data(data: Data) -> Possibles {
        let mut by_cells: HashMap<usize, HashSet<u8>> = HashMap::new();
        let mut by_values: HashMap<u8, HashSet<usize>> = HashMap::new();

        for i in 1..10 {
            by_values.insert(i, HashSet::new());
        }

        for (idx, val) in data.iter().enumerate() {
            match val {
                Some(_) => {},
                None => {
                    let mut possibles_set: HashSet<u8>  =HashSet::from_iter(1..10);

                    for oidx in Indices::around(idx) {
                        if let Some(val) = data[oidx] {
                            possibles_set.remove(&val);
                        }
                    }

                    by_cells.insert(idx, possibles_set);
                }
            }
        }

        // Invert by_cells to get by_values
        for (idx, vals) in by_cells.clone() {
            for val in vals {
                let hs = by_values.get_mut(&val).unwrap();
                hs.insert(idx);
            }
        }

        Possibles { by_cells, by_values}

    }

    fn update(&mut self, index: &Index, value: &Value ) {
        let hs = self.by_cells.get_mut(index).unwrap();
        hs.remove(value);
        if hs.is_empty() {
            self.by_cells.remove(&index);
        }

        let hs = self.by_values.get_mut(value).unwrap();
        hs.remove(index);
        
    }

    pub fn find_singles(&self) -> Vec<(Index, Value)>  {
        self.by_cells.iter()
            .filter(|(_, v)| v.len() == 1 )
            .map(|(k, v)| (*k, *v.iter().next().unwrap())).collect()
    }

}

struct Solver {
    data: Data,
    possibles: Possibles
}

impl Solver {
    fn new() -> Solver {
        let data = [None; 81];
        let possibles = Possibles::from_data(data);
        Solver {data, possibles}
    }

    fn from_file(filename: &str) -> Solver {
        let data = parse_from_file(&filename).expect("Error reading from file");
        let possibles = Possibles::from_data(data);
        //dbg!("{:?}", &possibles.by_cells);
        Solver { data, possibles }
    }

    fn remove_single_possibles(&mut self) {
        // Checks for any single possibles in the possibles map

        for (index, value) in self.possibles.find_singles() {
            self.update(index, value);
        }
    }

    fn update(&mut self, index: Index, value: Value) {
        self.data[index] = Some(value);
        self.possibles.update(&index, &value);

        for idx in Indices::around(index) {
            self.possibles.update(&idx, &value);
        }
    }

}
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


use std::error::Error;
fn parse_from_file(filename: &str) -> Result<Data, Box<dyn Error>>{
    let f = File::open(filename).expect("Unable to open file");

    let reader = BufReader::new(f);

    let mut data = [None; 81];

    for (n, line) in reader.lines().enumerate() {
        let line = line.expect(format!("Error reading line: {}", n).as_ref());
        println!("{:?}", line);
        let mut vals = line.split(",");

        let index = vals.next().unwrap().parse::<usize>()?;
        let value = vals.next().expect("Error splitting line value").parse::<u8>().expect("Unable to parse value");

        data[index] = Some(value);
    }

    Ok(data)
}

fn print_puzzle(data: Data) {
    
    for (idx, item) in data.iter().enumerate() {
        let val_str = match item {
            Some(value) => print!(" {} ", value.to_string()),
            None => print!("   ")
        };
        if idx % 9 == 0 && idx != 0{
            print!("\n");
        }
    }
}

fn main() {
    let filename = "puzzle1.txt";

    let solver = Solver::from_file(filename);
    
    print_puzzle(solver.data);
}
