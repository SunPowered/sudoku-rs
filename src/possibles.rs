use std::collections::{HashMap, HashSet};
use crate::{Data, Index, Value, Indices};

#[derive(Debug)]
pub struct Possibles {
    by_cells: HashMap<Index, HashSet<Value>>,
    by_values: HashMap<Value, HashSet<Index>>
}

impl Possibles {
    pub fn from_data(data: Data) -> Possibles {
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

    pub fn update(&mut self, index: &Index, value: &Value ) {
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