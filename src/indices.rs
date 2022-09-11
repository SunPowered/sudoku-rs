use std::iter::successors;
use crate::{Index, Position};

pub struct Indices {}

impl Indices {
    pub fn row(row: Index) -> Vec<Index> {
        successors(Some(9  * row), |i| Some(i+1)).take(9).collect()
    }

    pub fn column(column: Index) -> Vec<Index> {
        successors(Some(column), |i| Some(i+9)).take(9).collect()
    }

    pub fn subsquare(subsquare: Index) -> Vec<Index> {
        let start = 3 * (subsquare % 3) + 27 * (subsquare / 3);

        successors(Some(start), |i| Some(i+1)).take(3).chain(
            successors(Some(start + 9), |i| Some(i+1)).take(3).chain(
                successors(Some(start + 18), |i| Some(i+1)).take(3)
            )
        ).collect()
    }

    pub fn around(idx: usize) -> Vec<Index> {
        let position = Indices::position_from_index(idx);
        Indices::row(position.row).iter().cloned().chain(
            Indices::column(position.column).iter().cloned().chain(
                Indices::subsquare(position.subsquare).iter().cloned()
            )
        ).filter(|&i| i != idx).collect()
    }

    pub fn position_from_index(idx: Index) -> Position {
        let row = idx / 9;
        let column = idx % 9;
        let subsquare = 3 * (row / 3) + column / 3;   
        Position {row, column, subsquare}        
    }
}