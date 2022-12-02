use std::{fs, process::exit};

use aoc22::day1::solve_part_1;
use aoc22::day1::solve_part_2;

fn main() {
    let input_file_name = "input.txt";
    let input = fs::read_to_string(input_file_name).unwrap_or_else(|_| {
        eprintln!("failed to open input file '{}'", input_file_name);
        exit(1);
    });

    println!(
        "most calories carried by an individiual elf are {}",
        solve_part_1(&input)
    );
    println!(
        "most calories carried by an individiual elf are {}",
        solve_part_2(&input)
    );
}
