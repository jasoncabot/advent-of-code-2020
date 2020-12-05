use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let target = 2020_f64;
    let lines: Vec<f64> = read_expense_lines();

    let (a, b) = find_lines_for_target(lines, target).expect("No match found");

    println!("{} + {} = {}", a, b, a + b);
    println!("{} * {} = {}", a, b, a * b);
}

fn find_lines_for_target(expense_lines: Vec<f64>, target: f64) -> Result<(f64, f64), f64> {
    // loop through all numbers
    for (start_idx, amount1) in expense_lines.iter().enumerate() {
        // find target
        let current_target = target - amount1;

        // search from this number onwards because previous
        // numbers would have been already checked
        for current_idx in start_idx..expense_lines.len() {
            let value = expense_lines[current_idx];
            if value == current_target {
                return Ok((*amount1, value));
            }
        }
    }
    return Err(target);
}

fn read_expense_lines() -> Vec<f64> {
    let filename = "input.txt";
    let input = File::open(filename).expect("input.txt not found");
    let reader = BufReader::new(input);
    return reader
        .lines()
        .map(|l| l.expect("Can't read line").parse::<f64>().unwrap())
        .collect();
}
