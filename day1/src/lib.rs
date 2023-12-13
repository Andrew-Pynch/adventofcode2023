use std::fs::File;
use std::io::{self, BufRead};

pub fn get_lines_from_filename(filename: String) -> Result<Vec<String>, io::Error> {
    let path = filename;
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    return Ok(lines);
}

pub fn process_lines(lines: Vec<String>) -> i32 {
    let mut calibration_value_sum = 0;

    for line in lines.iter() {
        let mut numbers = Vec::new();

        for char in line.chars() {
            if char.is_digit(10) {
                numbers.push(char);
            }
        }

        if !numbers.is_empty() {
            let first_number = numbers[0];
            let last_number = *numbers.last().unwrap();

            let number_string = format!("{}{}", first_number, last_number);
            let number = number_string.parse::<i32>().unwrap();

            calibration_value_sum += number;
        }
    }

    return calibration_value_sum
}
