use std::collections::HashMap;

/*
https://adventofcode.com/2023/day/3

- Identify contiguous numbers in each row of a grid

Part 1: sum all numbers that are touching any symbol
Part 2: sum multiples of numbers whenever the * symbol touches exactly two numbers
*/
use gridthings::{Cell, Grid, GridFromString};

// Helper struct to represent contiguous Numbers in a Grid<char>
#[derive(Debug, Clone)]
pub struct Number {
    cells: Vec<Cell<char>>,
}

impl Number {
    pub fn new(cells: Vec<Cell<char>>) -> Self {
        Number { cells }
    }

    // Combine the char values of cells into a String then cast to i32
    pub fn value(&self) -> i32 {
        let mut s = String::new();
        for cell in &self.cells {
            s.push(cell.value);
        }
        s.parse::<i32>().unwrap()
    }

    // Find all adjacent cells to all the cells in this Number, not including cells that
    // are part of the Number itself, removing duplicates
    fn peek_all(&self, grid: &Grid<char>) -> Vec<Cell<char>> {
        let mut matches = Vec::new();
        for cell in &self.cells {
            let results = grid.peek_all(cell.y, cell.x, 1);
            for result in results {
                if !self.cells.contains(&result) && !matches.contains(&result) {
                    matches.push(result);
                }
            }
        }
        matches
    }

    // True if any of the cells are adjacent to a cell with a symbol (non-digit, non-.)
    pub fn symbol_adjacent(&self, grid: &Grid<char>) -> bool {
        for cell in self.peek_all(grid) {
            if !cell.value.is_digit(10) && cell.value != '.' {
                return true;
            }
        }
        false
    }

    // Return any adjacent cells that have value *
    pub fn gears(&self, grid: &Grid<char>) -> Vec<Cell<char>> {
        let mut gears = Vec::new();
        for cell in self.peek_all(grid) {
            if cell.value == '*' {
                gears.push(cell);
            }
        }
        gears
    }
}

fn main() {
    println!("Advent of Code 2023 Day 3");
    let text = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
    println!("Input:\n{}", text);
    let grid: Grid<char> = Grid::from_string(text);

    // Extract all Numbers
    let mut numbers: Vec<Number> = Vec::new();
    let mut current_collection = Vec::new(); // collect numeric characters to make a Number
    for row in grid.rows() {
        for cell in row {
            if cell.value.is_digit(10) {
                current_collection.push(cell.clone());
            } else if !current_collection.is_empty() {
                numbers.push(Number::new(current_collection.clone()));
                current_collection.clear();
            }
        }
        if !current_collection.is_empty() {
            numbers.push(Number::new(current_collection.clone()));
            current_collection.clear();
        }
    }

    println!("Gathered {} numbers\n", numbers.len());
    println!("Part 1: Identify numbers touching a symbol, and sum their value");
    let mut p1_sum = 0;
    for number in &numbers {
        let is_adjacent = number.symbol_adjacent(&grid);
        println!(
            "Number: {}. Symbol Adjacent? {}",
            &number.value(),
            &is_adjacent
        );
        if is_adjacent {
            p1_sum += number.value();
        }
    }
    println!("\nPart 1: Answer: {}", p1_sum);

    println!("\nPart 2: Identify gears touching exactly two numbers, sum their products.");
    // Find all * in the grid touching exactly two numbers by sort of searching in reverse,
    // make a HashMap where the Cell with a * is the key and iterate through all Numbers adding the
    // Number obj to that Cell's value (Vec) then look for entries with only two hits
    let mut gears: HashMap<Cell<char>, Vec<Number>> = HashMap::new();
    for number in &numbers {
        for gear in number.gears(&grid) {
            gears.entry(gear).or_insert(Vec::new()).push(number.clone());
        }
    }

    let mut p2_sum = 0;
    for (cell, numbers) in gears {
        println!("{:?} touching {} numbers", cell, numbers.len());
        if numbers.len() == 2 {
            p2_sum += numbers[0].value() * numbers[1].value();
        }
    }
    println!("\nPart 2: Answer: {}", p2_sum);
}
