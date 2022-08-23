use arrayvec::ArrayVec;
use crate::{Indices, Index};

type CachedSubset = ArrayVec<Indices, 9>;
pub struct CachedIndices {
    pub rows: CachedSubset,
    pub columns: CachedSubset,
    pub subsquares: CachedSubset
}

impl CachedIndices {
    pub fn new() -> CachedIndices{
        return CachedIndices {
            rows: CachedIndices::generate_indices(CachedIndices::row_indices),
            columns: CachedIndices::generate_indices(CachedIndices::column_indices),
            subsquares: CachedIndices::generate_indices(CachedIndices::subsquare_indices)
        }
    }

    fn row_indices(row_index: Index) -> Indices {
        (0..9).map(|i| 9 * row_index + i).collect()
    }
    
    fn column_indices(column_index: Index) -> Indices {
        (0..9).map(|i| column_index + 9 * i).collect()
    }
    
    fn subsquare_indices(subsquare_index: Index) -> Indices {
    
        let mut subsquare = ArrayVec::from([0; 9]);
        let row = subsquare_index / 3;
        let column = subsquare_index % 3;
        let start = 27 * row + 3 * column;
        for i in 0..3 {
            for j in 0..3 {
                subsquare[3*i+j] = start + 9 * i + j;
            }
        }
        subsquare
    }
    
    fn generate_indices(f: fn(usize) -> Indices) -> CachedSubset {
        (0..9).map(|i| f(i)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_row_indices() {
        let row_idxs = CachedIndices::row_indices(4);
        assert_eq!(row_idxs, ArrayVec::from([36, 37, 38, 39, 40, 41, 42, 43, 44]));
    }
    
    #[test]
    fn test_column_indices() {
        let col_idxs = CachedIndices::column_indices(3);
        assert_eq!(col_idxs, ArrayVec::from([3, 12, 21, 30, 39, 48, 57, 66, 75]));
    }
    
    #[test]
    fn test_subsquare_indices() {
        let subsquare_idxs = CachedIndices::subsquare_indices(2);
        assert_eq!(subsquare_idxs, ArrayVec::from([6, 7, 8, 15, 16, 17, 24, 25, 26]));
    }
    
}