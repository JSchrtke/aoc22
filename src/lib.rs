pub mod day1 {
    pub fn solve_part_1(input: &str) -> u32 {
        let mut sums = parse_to_sorted_sums(input);
        sums.sort();
        *sums.iter().last().unwrap()
    }

    pub fn solve_part_2(input: &str) -> u32 {
        let mut sums = parse_to_sorted_sums(input);
        sums.sort();
        sums.into_iter().rev().take(3).sum()
    }

    fn parse_to_sorted_sums(input: &str) -> Vec<u32> {
        input
            .split("\n\n")
            .map(|chunk| {
                chunk
                    .split('\n')
                    .filter(|line| !line.is_empty())
                    .map(|line| line.parse::<u32>().unwrap_or(0))
                    .sum()
            })
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use crate::day1::solve_part_1;
        use crate::day1::solve_part_2;

        #[test]
        fn part_1() {
            let input = "\n1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n\n";
            let expected = 24000;

            let actual = solve_part_1(&input);

            assert_eq!(expected, actual);
        }

        #[test]
        fn part_2() {
            let input = "\n1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n\n";
            let expected = 45000;

            let actual = solve_part_2(&input);

            assert_eq!(expected, actual);
        }
    }
}
