use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let lines: Vec<f64> = read_expense_lines_sorted();

    // let lines = find_lines_for_target(&lines, 1558_f64, 1).expect("No match found");
    // let lines = find_lines_for_target(&lines, 2020_f64, 2).expect("No match found");
    // let lines = find_lines_for_target(&lines, 2020_f64, 3).expect("No match found");
    // let lines = find_lines_for_target(&lines, 7255_f64, 4).expect("No match found");
    let lines = find_lines_for_target(&lines, 4768_f64, 5).expect("No match found");

    for (index, value) in lines.iter().enumerate() {
        println!("Line {}: {}", index, value);
    }
}

fn find_lines_for_target(
    expense_lines: &[f64],
    target: f64,
    count: usize,
) -> Result<Vec<f64>, f64> {
    println!(
        "Looking in {} lines for target {} from {} items",
        expense_lines.len(),
        target,
        count
    );

    if count == 1 {
        match expense_lines.binary_search_by(|val| target.partial_cmp(&val).unwrap()) {
            Ok(val) => return Ok(vec![expense_lines[val]]),
            Err(_) => return Err(target),
        }
    } else {
        // Find the first largest number below target
        let idx_start = expense_lines
            .binary_search_by(|b| target.partial_cmp(&b).unwrap())
            .unwrap_or_else(|x| x);

        for idx1 in idx_start..(expense_lines.len() - (count - 1)) {
            let left = expense_lines[idx1];

            if count == 2 {
                // base case - find a pair in the remaining slice
                for idx2 in idx1 + 1..expense_lines.len() {
                    let right = expense_lines[idx2];

                    if left + right == target {
                        return Ok(vec![left, right]);
                    }
                }
            } else {
                // find n lines that sum to the total remaining recursively
                // and if we get a result, add on our current value and this is the answer
                match find_lines_for_target(&expense_lines[idx1 + 1..], target - left, count - 1) {
                    Ok(child_results) => {
                        let mut results: Vec<f64> = Vec::new();
                        results.push(left);
                        for result in child_results {
                            results.push(result);
                        }
                        return Ok(results);
                    }
                    Err(_) => (), // do nothing - just backtrack
                }
            }
        }
    }

    return Err(target);
}

fn read_expense_lines_sorted() -> Vec<f64> {
    let filename = "input.txt";
    let input = File::open(filename).expect("input.txt not found");
    let reader = BufReader::new(input);
    let mut lines: Vec<f64> = reader
        .lines()
        .map(|l| l.expect("Can't read line").parse::<f64>().unwrap())
        .collect();
    // Sort descending so that we can optimise searching
    lines.sort_by(|a, b| b.partial_cmp(a).unwrap());
    return lines;
}
