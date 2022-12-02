use std::{fs, process::exit};

use aoc22::day1::find_biggest_sum;

fn main() {
    let input_file_name = "input.txt";
    let calorie_list = fs::read_to_string(input_file_name).unwrap_or_else(|_| {
        eprintln!("failed to open input file '{}'", input_file_name);
        exit(1);
    });

    match find_biggest_sum(&calorie_list) {
        Ok(most_calories) => println!("most calories carried are {}", most_calories),
        Err(err) => eprintln!("error finding most calories: {}", err),
    }
}
