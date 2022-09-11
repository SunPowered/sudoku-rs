use std::collections::{HashMap, HashSet};
use itertools::sorted;

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

    pub fn len(&self) -> usize {
        self.by_cells.len()
    }
    pub fn remove(&mut self, index: &Index) {
        self.by_cells.remove(index);
    }

    pub fn update(&mut self, index: &Index, value: &Value ) {
        match self.by_cells.get_mut(index) {
            Some(hs) => {
                
                match hs.remove(value) {
                    true => (),
                    false => ()
                }},
            None => ()
        }
        
        match self.by_values.get_mut(value) {
            Some(hs) => match hs.remove(index) {
                true => (),
                false => ()
            },
            None => ()
        }
        
    }

    pub fn find_singles(&self) -> Vec<(Index, Value)>  {
        self.by_cells.iter()
            .filter(|(_, v)| v.len() == 1 )
            .map(|(k, v)| (*k, *v.iter().next().unwrap())).collect()
    }

    pub fn window(&self, window: Vec<Index>) -> HashMap<Value, HashSet<Index>> {
        let window_hs: HashSet<Index> = HashSet::from_iter(window);
        let mut out_map: HashMap<Value, HashSet<Index>> = HashMap::new();
        self.by_values.iter().for_each(|(k, v)| {
            let intersection_set: HashSet<Index> = v.intersection(&window_hs).copied().collect();
            if !intersection_set.is_empty() {
                out_map.insert(*k, intersection_set);
            }
        });
        out_map

    }

    pub fn print(&self) {
        println!("By Indices");

        for idx in sorted(self.by_cells.keys()) {
            println!("{} -> {:?}", idx, self.by_cells.get(idx).unwrap());
        }

        println!("By Values");
        for val in sorted(self.by_values.keys()) {
            println!("{} -> {:?}", val, self.by_values.get(val).unwrap());
        }
    }

}