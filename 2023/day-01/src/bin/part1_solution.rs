fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> u32 {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| character.to_digit(10));
            let first = it.next().expect("No first digit");

            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("Could not parse number")
        })
        .sum::<u32>();

    output
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn test_solution() {
        let input = include_str!("./inputs/input1_test.txt");
        let output = solution(input).to_string();
        let expected_output = include_str!("./outputs/output1_test.txt");

        assert_eq!(output, expected_output);
    }
}
