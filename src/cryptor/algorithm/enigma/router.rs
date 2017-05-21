
/// enigma module
use super::{ SubstitutionTable, ALPHABETS };

/// utility module
use utility::{ GetIndex, IndexResult };

pub enum Digit {
    Up,
    NoChange
}

pub struct Router {
    position: usize,
    substitution_table: SubstitutionTable
}

pub trait RouterProtocol {
    fn increment(&mut self) -> Digit;
    fn crypto(&self, character: &char) -> char;
    fn reverse(&self, character: &char) -> char;
}

impl RouterProtocol for Router {

    fn increment(&mut self) -> Digit {
        self.position += 1;
        let digit     = self.get_digit();
        self.position = self.position % self.get_length();

        return digit
    }

    fn crypto(&self, character: &char) -> char {

        let characters = ALPHABETS.to_vec();
        let length     = self.get_length();

        match characters.get_index(character) {
            IndexResult::Exist(index) => return self.substitution_table.characters[(index + self.position) % length],
            IndexResult::None         => *character
        }
    }

    fn reverse(&self, character: &char) -> char {

        let characters = ALPHABETS.to_vec();
        let length     = self.get_length();

        match self.substitution_table.characters.get_index(character) {
            IndexResult::Exist(index) => characters[(length + index - self.position) % length],
            IndexResult::None         => *character
        }
    }
}

impl Router {

    pub fn new(substitution_table: SubstitutionTable) -> Self {
        Router {
            position: 0,
            substitution_table
        }
    }

    pub fn set_position(&mut self, position: &char) {
        self.position = match ALPHABETS.to_vec().get_index(position) {
            IndexResult::Exist(index) => index,
            IndexResult::None         => 0
        }
    }

    pub fn get_position(&self) -> usize {
        self.position
    }

    fn get_length(&self) -> usize {
        self.substitution_table.characters.len()
    }

    fn get_digit(&self) -> Digit {
        match self.position == self.substitution_table.characters.len() {
            true  => Digit::Up,
            false => Digit::NoChange
        }
    }
}
