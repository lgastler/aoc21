use std::fs;

pub fn run() {
    let input: String = load_input();
    let input_array: Vec<&str> = input.split("\n").collect();
    let input_array_int: Vec<i32> = input_array.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let result = count_larger_than_current(input_array_int);
    println!("{}", result);
}

// load input.txt into a string
fn load_input() -> String {
    let filename = fs::canonicalize("src/day01/input.txt").unwrap();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return contents;
}

// map over array and count if next value is larger than current value
fn count_larger_than_current(input: Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    for i in 0..input.len() {
        if i < input.len() - 1 {
            if input[i] < input[i + 1] {
                count += 1;
            }
        }
    }
    return count;
}