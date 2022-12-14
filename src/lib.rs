pub mod day1 {
    use std::{fmt::Display, num::ParseIntError};

    pub fn find_biggest_sum(input: &str) -> Result<u32, BadInputError> {
        if input.is_empty() {
            return Err(BadInputError::Empty);
        }

        let mut current_sum = 0;
        let mut biggest_sum = 0;
        for line in input.split('\n') {
            if line.is_empty() {
                if current_sum > biggest_sum {
                    biggest_sum = current_sum;
                }
                current_sum = 0;
                continue;
            }

            current_sum += line.parse::<u32>()?;
        }
        if current_sum > biggest_sum {
            biggest_sum = current_sum;
        }

        Ok(biggest_sum)
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum BadInputError {
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
        use crate::day1::{find_biggest_sum, BadInputError};

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
            let input = String::from("1\n\n3\n\n2\n\n");
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
}
