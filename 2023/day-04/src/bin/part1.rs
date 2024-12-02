fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> u32 {
    let output = input
        .lines()
        .map(|line| {
            let number_parts = line
                .split(":")
                .last()
                .expect("No number part")
                .split("|")
                .collect::<Vec<_>>();

            let winning_numbers = number_parts[0]
                .split_whitespace()
                .map(|number| number.parse::<u32>().expect("Not a number"))
                .collect::<Vec<_>>();

            let n_matches = number_parts[1]
                .split_whitespace()
                .map(|number_str| {
                    let num = number_str.parse::<u32>().expect("Not a number");

                    winning_numbers.contains(&num) as u32
                })
                .sum::<u32>();

            match n_matches {
                0 => 0,
                _ => u32::pow(2, n_matches - 1),
            }
        })
        .sum::<u32>();

    output
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn test_solution() {
        let input = include_str!("./tests/input1.txt");
        let output = solution(input).to_string();
        let expected_output = include_str!("./tests/output1.txt");

        assert_eq!(output, expected_output);
    }
}
