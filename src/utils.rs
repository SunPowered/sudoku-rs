use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;

use crate::{Data, Index};

pub fn parse_from_file(filename: &str) -> Result<Data, Box<dyn Error>>{
    let f = File::open(filename).expect("Unable to open file");

    let reader = BufReader::new(f);

    let mut data = [None; 81];

    for (n, line) in reader.lines().enumerate() {
        let line = line.expect(format!("Error reading line: {}", n).as_ref());
        //println!("{:?}", line);
        let mut vals = line.split(",");

        let index = vals.next().unwrap().parse::<usize>()?;
        let value = vals.next().expect("Error splitting line value").parse::<u8>().expect("Unable to parse value");

        data[index] = Some(value);
    }

    Ok(data)
}

pub fn print_puzzle_(data: Data) {
    
    for (idx, item) in data.iter().enumerate() {
        match item {
            Some(value) => print!(" {} ", value.to_string()),
            None => print!("   ")
        };
        if idx % 9 == 0 && idx != 0{
            print!("\n");
        }
    }
}

pub fn print_puzzle(data: &Data ) {
    let horizontal_line ="+ - + - + - + - + - + - + - + - + - +";
    
    for row_index in 0..9 {
        if (row_index % 3 == 0) && (row_index != 0) {
            println!("{}", horizontal_line);
        }
        print_line(data, row_index)
    }
    // println!("{}", horizontal_line);
}

fn print_line(data: &Data, row_index: Index) {
    let no_value = " ";
    let row_vals = &data[9*row_index..9*row_index+9];

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