use std::fs;

pub fn run() {
    println!("Running exercise1");
    let input_array = load_input();
    let result = calculate_movements(input_array);
    println!("Result: {}", result);
}

fn load_input() -> Vec<String> {
    let filename = fs::canonicalize("src/day02/input.txt").unwrap();
    let raw_data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = raw_data.lines().collect();
    return lines.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
}

// sum amount of movements for forward, down and up from string
fn calculate_movements(array: Vec<String>) -> i32 {
    let mut aim = 0;
    let mut horizontal_movements = 0;
    let mut vertical_movements = 0;

    for (_, line) in array.iter().enumerate() {
        if line.contains("forward") {
            horizontal_movements += get_number_from_string(line);
            vertical_movements += aim * get_number_from_string(line)
        } else if line.contains("up") {
            aim -= get_number_from_string(line)
        } else if line.contains("down") {
            aim += get_number_from_string(line)
        }
    }

    return horizontal_movements * vertical_movements;
}

// split string at space and return number from second part
fn get_number_from_string(string: &str) -> i32 {
    return string.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
}