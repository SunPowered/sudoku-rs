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

        for val in 1..10 {
            if let Some(idxs) = self.by_values.get_mut(&val) {
                idxs.remove(index);
            }
        }
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

    pub fn find_single_values(&self) -> Vec<(Index, Value)> {
        let mut singles: Vec<(Index, Value)> = vec!();
        for i in 0..9 {
            for f in [Indices::row, Indices::column, Indices::subsquare] {
                self.window(f(i)).iter().filter(|(_, v)| v.len() == 1)
                    .map(|(k, v)| (v.iter().next().unwrap(), k))
                    .for_each( |(&k, &v)|{
                        singles.push((k, v));
                    });
            }
        }
        singles
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
    fn find_overlays(&mut self, subsquare_idxs: &HashSet<Index>, other_idxs: HashSet<Index>) {
        let inner_idxs: HashSet<Index> = HashSet::from_iter(subsquare_idxs.intersection(&other_idxs).cloned());
        let outer_subsquare: HashSet<Index> = HashSet::from_iter(subsquare_idxs.difference(&other_idxs).cloned());
        let outer_other: HashSet<Index> = HashSet::from_iter(other_idxs.difference(&subsquare_idxs).cloned());        

        let vals_inner = self.window(Vec::from_iter(inner_idxs));
        let vals_outer_subsquare = self.window(Vec::from_iter(outer_subsquare.clone()));
        let vals_outer_other = self.window(Vec::from_iter(outer_other.clone()));

        for (val, _) in vals_inner {
            let outer_subsquare_has_val = vals_outer_subsquare.contains_key(&val);
            let outer_other_has_val = vals_outer_other.contains_key(&val);

            if !outer_other_has_val {
                //println!("Removing overlay from subsquare: {} - {:?}", val, outer_subsquare);
                for idx in outer_subsquare.clone() {
                    self.update(&idx, &val);
                }
            }

            if !outer_subsquare_has_val {
                //println!("Removing overlay from other: {} - {:?}", val, outer_other);
                for idx in outer_other.clone() {
                    self.update(&idx, &val);
                }
            }
        }

    }
    pub fn remove_overlays(&mut self) {
        let rows_idxs = (0..9).map(|i| Indices::row(i)).collect::<Vec<Vec<Index>>>();
        let columns_idxs = (0..9).map(|i| Indices::column(i)).collect::<Vec<Vec<Index>>>();

        for subsquare_id in 0..9 {
            let subsquare_idxs: HashSet<Index> = HashSet::from_iter(Indices::subsquare(subsquare_id));

            let super_row = subsquare_id / 3;
            let super_column = subsquare_id % 3;

            for row_id in 0..3 {
                let row_ids: HashSet<Index> = HashSet::from_iter(rows_idxs[(3 * super_row) + row_id].clone());    
                self.find_overlays(&subsquare_idxs, row_ids);
                
                let column_ids: HashSet<Index> = HashSet::from_iter(columns_idxs[(3 * super_column) + row_id].clone());
                self.find_overlays(&subsquare_idxs, column_ids)
            }
        }
    }

    pub fn print(&self) {
        println!("By Indices");

        for idx in sorted(self.by_cells.keys()) {
            println!("{} -> {:?}", idx, sorted(self.by_cells.get(idx).unwrap()).collect::<Vec<&Value>>());
        }

        println!("By Values");
        for val in sorted(self.by_values.keys()) {
            println!("{} -> {:?}", val, sorted(self.by_values.get(val).unwrap()).collect::<Vec<&Index>>());
        }
    }

}