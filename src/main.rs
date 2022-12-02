use std::{fmt::Display, fs, num::ParseIntError, process::exit};

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

fn find_biggest_sum(input: &str) -> Result<u32, BadInputError> {
    if input.is_empty() {
        return Err(BadInputError::Empty);
    }

    let blocks = input
        .split("\n\n")
        .filter(|item| !item.is_empty())
        .collect::<Vec<&str>>();

    let mut sums: Vec<u32> = Vec::new();
    for block in blocks {
        let number_strs = block
            .split('\n')
            .filter(|item| !item.is_empty())
            .collect::<Vec<&str>>();
        let mut sum = 0;
        for number_str in number_strs {
            sum += number_str.parse::<u32>()?;
        }
        sums.push(sum);
    }

    let mut biggest_sum = 0u32;
    for sum in sums {
        if sum > biggest_sum {
            biggest_sum = sum
        }
    }

    Ok(biggest_sum)
}

#[derive(Debug, PartialEq)]
enum BadInputError {
    Empty,
    InvalidData(String),
}

impl Display for BadInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadInputError::Empty => write!(f, "cannot read empty input"),
            BadInputError::InvalidData(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<ParseIntError> for BadInputError {
    fn from(err: ParseIntError) -> Self {
        BadInputError::InvalidData(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{find_biggest_sum, BadInputError};

    #[test]
    fn empty_input_returns_error() {
        let input = String::new();
        let output = find_biggest_sum(&input);
        assert_eq!(Err(BadInputError::Empty), output);
    }

    #[test]
    fn non_number_in_data_returns_error() {
        let input = String::from("abc");
        let output = find_biggest_sum(&input);
        assert_eq!(
            Err(BadInputError::InvalidData(String::from(
                "invalid digit found in string"
            ))),
            output
        );
    }

    #[test]
    fn single_number_is_result() {
        let input = String::from("4000");
        let output = find_biggest_sum(&input).unwrap();
        assert_eq!(4000, output);
    }

    #[test]
    fn multiple_consecutive_numbers_get_summed_up() {
        let input = String::from("1\n2");
        let output = find_biggest_sum(&input).unwrap();
        assert_eq!(3, output);
    }

    #[test]
    fn of_multiple_separate_numbers_biggest_is_returned() {
        let input = String::from("1\n\n2\n\n3\n\n");
        let output = find_biggest_sum(&input).unwrap();
        assert_eq!(3, output);
    }

    #[test]
    fn block_can_start_with_newline() {
        let input = String::from("1\n\n2\n\n\n3\n\n");
        let output = find_biggest_sum(&input).unwrap();
        assert_eq!(3, output);
    }
}
