# Sudoku Solver

This project is a small Sudoku puzzle solver written in Rust.  It's intended to help me learn more depth 
into the language, and scratch an itch in problem solving algorithms.

## Puzzle board

There are a few ways to approach the problem space.  I decided on a single 81 length array of `Option<u8>` 
elements.  The board is zero-indexed from the top left of the board, counting rows to the right.  

The `Indices` struct is able to generate the appropriate index windows (row, column, subsquare) for the 
solution algorithm.  These indices are also used in the initial data file.

A simple map of the board indices is given as:

             0 | ...  | 8
             9 | ...  | 17
                 .
                 .
                 .
             72| ...  | 80      

## File Input

Currently, a simple text file is used to input the puzzle data to the program.

The input file is any simple ascii text file.  Each puzzle entry is a line denoting the 
index and value of the entry.  The indices are defined above, (0, 80) from the top left.

Example: 

```
0,6
4,1
5,7
7,9
```

would give the first row as 

            | 6 |   |   |   | 1 | 7 |   | 9 |   |
    Index-->  0   1   2   3   4   5   6   7   8

## Usage

Run the script with the input puzzle file:

`$ cargo run <puzzle_path>`

It will try to solve the puzzle and print the result to the terminal