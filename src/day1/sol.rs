use std::fs::File;
use std::io::{BufRead, BufReader};

fn extract_numbers(input: String) -> i32 {
    let numbers: String = input.chars().filter(|c| c.is_numeric()).collect();
    let first_number = numbers.chars().next().unwrap();
    let last_number = numbers.chars().last().unwrap();
    let digit: i32 = format!("{}{}", first_number, last_number).parse().unwrap();
    return digit;
}

fn main() {
    let file = File::open("src/day1/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum: i32 = 0;

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let number = extract_numbers(line);
        sum += number;
    }

    return println!("{}", sum);
}
