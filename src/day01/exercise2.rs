use std::fs;

pub fn run() {
    let input: String = load_input();
    let input_array: Vec<&str> = input.split("\n").collect();
    let input_array_int: Vec<i32> = input_array.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let result = count_larger_than_previous(input_array_int);
    println!("{}", result);
}

// load input.txt into a string
fn load_input() -> String {
    let filename = fs::canonicalize("src/day01/input.txt").unwrap();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return contents;
}

// map over the array always add the next 3 elements and compare to the next 3 elements
fn count_larger_than_previous(input_array: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for i in 0..input_array.len() {
        if i + 3 < input_array.len() {
            if input_array[i] + input_array[i + 1] + input_array[i + 2] < input_array[i + 1] + input_array[i + 2] + input_array[i + 3] {
                result += 1;
            }
        }
    }
    return result;
}