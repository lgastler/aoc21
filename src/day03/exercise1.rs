use std::collections::HashMap;
use std::fs;

pub fn run() {
    println!("Running exercise1");
    let input_array = load_input();
    let gamma_rate = calculate_gamma(input_array);
    let epsilon_rate = invert_binary(&gamma_rate);
    let gamma_rate_number = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate_number = isize::from_str_radix(&epsilon_rate, 2).unwrap();
    println!("Result: {}", gamma_rate_number * epsilon_rate_number);
}

fn load_input() -> Vec<String> {
    let filename = fs::canonicalize("src/day03/input.txt").unwrap();
    let raw_data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = raw_data.lines().collect();
    return lines
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
}

// input: array of strings
// output: single character
// find the most used character in all the strings at one specified index
fn calculate_gamma(input_array: Vec<String>) -> String {
    let binary_length = input_array[0].len();
    let mut res: Vec<char> = Vec::new();
    for i in 0..binary_length {
        let mut movements: Vec<char> = Vec::new();
        for line in &input_array {
            let chars: Vec<char> = line.chars().collect();
            movements.push(chars[i]);
        }
        let mut counts: HashMap<char, i32> = HashMap::new();
        for movement in movements {
            let count = counts.entry(movement).or_insert(0);
            *count += 1;
        }
        let mut max_count = 0;
        let mut max_char = ' ';
        for (key, value) in counts {
            if value > max_count {
                max_count = value;
                max_char = key;
            }
        }
        res.push(max_char)
    }
    let res_string: String = res.iter().collect();
    return res_string;
}

fn invert_binary(binary: &String) -> String {
    let mut inverted = String::new();
    for c in binary.chars() {
        if c == '1' {
            inverted.push_str("0");
        } else {
            inverted.push_str("1");
        }
    }
    return inverted;
}
