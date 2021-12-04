use std::fs;

const LINE_LENGTH: isize = 12;

pub fn run() {
    println!("Running exercise2");
    let input_array = load_input();
    let oxygen_binary = calculate_value(input_array.clone(), char::from('1'), true);
    println!("Oxygen binary: {}", oxygen_binary);
    let co2_binary = calculate_value(input_array.clone(), char::from('0'), false);
    println!("CO2 binary: {}", co2_binary);
    let oxygen = isize::from_str_radix(&oxygen_binary, 2).unwrap();
    let co2 = isize::from_str_radix(&co2_binary, 2).unwrap();
    println!("Result: {}", oxygen * co2);
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

fn calculate_value(input_array: Vec<String>, fallback_char: char, greatest: bool) -> String {
    let mut filter_string = String::new();
    let mut filtered_array: Vec<String> = input_array.clone();
    let mut result_string = String::new();
    // iterate over from 0 to LINE_LENGTH
    for i in 0..LINE_LENGTH {
        let selection_char_for_pos = get_greatest_or_lowest_char_for_position(
            filtered_array.clone(),
            i,
            greatest,
            fallback_char,
        );
        filter_string.push(selection_char_for_pos);
        filtered_array = filter_array(filtered_array.clone(), filter_string.clone());

        if filtered_array.len() == 1 {
            result_string = filtered_array[0].clone();
            break;
        }
    }

    return result_string;
}

// get the caracter that appears the most at the specified position
fn get_greatest_or_lowest_char_for_position(
    input_array: Vec<String>,
    position: isize,
    greatest: bool,
    fallback_char: char,
) -> char {
    let mut chars_at_position: Vec<char> = Vec::new();
    for line in input_array.iter() {
        let mut chars = line.chars();
        chars_at_position.push(chars.nth(position as usize).unwrap());
    }

    let zero_chars = count_chars(chars_at_position.clone(), '0');
    let one_chars = count_chars(chars_at_position.clone(), '1');
    if zero_chars == one_chars {
        return fallback_char;
    }

    if greatest {
        return if one_chars > zero_chars { '1' } else { '0' };
    } else {
        return if one_chars < zero_chars { '1' } else { '0' };
    }
}

// count the amount of chars in a given char array
fn count_chars(chars: Vec<char>, target: char) -> isize {
    let mut count: isize = 0;
    for c in chars.iter() {
        if *c == target {
            count += 1;
        }
    }
    return count;
}

// filter array of string that match the filter string
fn filter_array(input_array: Vec<String>, filter_string: String) -> Vec<String> {
    let mut filtered_array: Vec<String> = Vec::new();
    for line in input_array.iter() {
        if line.starts_with(&filter_string) {
            filtered_array.push(line.to_string());
        }
    }
    return filtered_array;
}
