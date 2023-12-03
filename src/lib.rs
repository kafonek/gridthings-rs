use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cell<T>
where
    T: Clone + PartialEq + Eq + Hash,
{
    pub y: usize, // row number, 0 is top row
    pub x: usize, // column number, 0 is left-most entry
    pub value: T,
}

#[derive(Debug)]
pub struct Grid<T>
where
    T: Clone + PartialEq + Eq + Hash,
{
    data: Vec<Vec<Cell<T>>>,
}

pub trait GridFromString<T> {
    fn from_string(text: &str) -> Self;
}

impl GridFromString<char> for Grid<char> {
    fn from_string(text: &str) -> Self {
        let mut data = Vec::new();
        for (y, line) in text.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                row.push(Cell { y, x, value: c });
            }
            data.push(row);
        }
        Grid { data }
    }
}

impl GridFromString<i32> for Grid<i32> {
    fn from_string(text: &str) -> Self {
        let mut data = Vec::new();
        for (y, line) in text.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                // Give error message including character if we can't coerce to i32
                let value = c
                    .to_digit(10)
                    .expect(&format!("Failed to cast to i32: {}", c))
                    as i32;
                row.push(Cell { y, x, value });
            }
            data.push(row);
        }
        Grid { data }
    }
}

impl<T> Grid<T>
where
    T: Clone + PartialEq + Eq + Hash,
{
    pub fn rows(&self) -> impl Iterator<Item = &Vec<Cell<T>>> {
        self.data.iter()
    }

    pub fn get(&self, y: usize, x: usize) -> Option<&Cell<T>> {
        self.data.get(y).and_then(|row| row.get(x))
    }

    // Given a starting coordinate, peek left and right by a given offset returning existing cells
    pub fn peek_horizontal(&self, y: usize, x: usize, offset: usize) -> Vec<Cell<T>> {
        let mut results = Vec::new();
        // avoid underflow when peeking left (x value getting negative)
        if let Some(look_left) = x.checked_sub(offset) {
            if let Some(cell) = self.get(y, look_left) {
                results.push(cell.clone());
            }
        }
        if let Some(cell) = self.get(y, x + offset) {
            results.push(cell.clone());
        }
        results
    }

    // Given a starting coordinate, peek up and down by a given offset
    pub fn peek_vertical(&self, y: usize, x: usize, offset: usize) -> Vec<Cell<T>> {
        let mut results = Vec::new();
        // avoid underflow when peeking up (y value getting negative)
        if let Some(look_up) = y.checked_sub(offset) {
            if let Some(cell) = self.get(look_up, x) {
                results.push(cell.clone());
            }
        }
        if let Some(cell) = self.get(y + offset, x) {
            results.push(cell.clone());
        }
        results
    }

    // Given a starting coordinate, peek up, down, left, and right by a given offset
    pub fn peek_linear(&self, y: usize, x: usize, offset: usize) -> Vec<Cell<T>> {
        let mut results = Vec::new();
        results.extend(self.peek_horizontal(y, x, offset));
        results.extend(self.peek_vertical(y, x, offset));
        results
    }

    // Given a starting coordinate, peek diagonal by a given offset
    pub fn peek_diagonal(&self, y: usize, x: usize, offset: usize) -> Vec<Cell<T>> {
        let mut results = Vec::new();
        // avoid underflow when peeking up or left (y or x value getting negative)
        let look_up = y.checked_sub(offset);
        let look_left = x.checked_sub(offset);
        // Check up and left
        if look_up.is_some() && look_left.is_some() {
            if let Some(cell) = self.get(look_up.unwrap(), look_left.unwrap()) {
                results.push(cell.clone());
            }
        }
        // Check up and right
        if look_up.is_some() {
            if let Some(cell) = self.get(look_up.unwrap(), x + offset) {
                results.push(cell.clone());
            }
        }
        // Check down and left
        if look_left.is_some() {
            if let Some(cell) = self.get(y + offset, look_left.unwrap()) {
                results.push(cell.clone());
            }
        }
        // Check down and right
        if let Some(cell) = self.get(y + offset, x + offset) {
            results.push(cell.clone());
        }
        results
    }

    // Given a starting coordinate, peek up, down, left, right, and diagonal by a given offset
    pub fn peek_all(&self, y: usize, x: usize, offset: usize) -> Vec<Cell<T>> {
        let mut results = Vec::new();
        results.extend(self.peek_linear(y, x, offset));
        results.extend(self.peek_diagonal(y, x, offset));
        results
    }
}
