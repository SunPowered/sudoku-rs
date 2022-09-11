use crate::{Data, Index, Value, Indices, Possibles};
pub struct Solver {
    data: Data,
    possibles: Possibles
}

impl Solver {
    pub fn new(data: Data) -> Solver {
        let possibles = Possibles::from_data(data);
        Solver {data, possibles}
    }

    pub fn data(&self) -> Data {
        self.data.clone()
    }
    
    fn remove_single_possibles(&mut self) {
        // Checks for any single possibles in the possibles map

        for (index, value) in self.possibles.find_singles() {
            self.update(index, value);
        }
    }

    fn update(&mut self, index: Index, value: Value) {
        dbg!("Updating solution [{}] = {}", index, value);
        self.data[index] = Some(value);
        self.possibles.update(&index, &value);

        for idx in Indices::around(index) {
            self.possibles.update(&idx, &value);
        }
    }

    pub fn solve(&mut self) {
        self.remove_single_possibles();
    }

}