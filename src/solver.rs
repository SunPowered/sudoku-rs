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

    fn remove_single_values(&mut self) {
        // For each row, column, subsquare, check for any single possible values

        for (index, value) in self.possibles.find_single_values() {
            self.update(index, value);
        }
        
    }

    fn update(&mut self, index: Index, value: Value) {
        if let Some(_) = self.data[index] {return};

        println!("Updating solution [{}] = {}", index, value);
        self.data[index] = Some(value);
        self.possibles.remove(&index);

        for idx in Indices::around(index) {
            self.possibles.update(&idx, &value);
        }
    }

    pub fn solve(&mut self) {
        println!("Solving");
        
        let mut _possibles_count = self.possibles.len();
        let mut count: usize = 0;
        println!("# possibles - start: {}", self.possibles.len());
        loop {
            self.remove_single_possibles();
            self.remove_single_values();
            count += 1;
            
            println!("# possibles Pass: {}, Count: {}", count, self.possibles.len());
            
            if self.possibles.len() == 0 || self.possibles.len() == _possibles_count {
                break
            }
            _possibles_count = self.possibles.len();
        }

        println!("Finished after {} passes", count);
        if self.possibles.len() != 0 {
            self.possibles.print();
        }
        
    }

}