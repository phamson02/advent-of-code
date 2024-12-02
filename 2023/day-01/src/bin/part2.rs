fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> i32 {
    let digits: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum: i32 = 0;
    for line in input.lines() {
        let mut first_number: i32 = 0;
        let mut last_number: i32 = 0;

        for start_idx in 0..line.len() {
            let mut num: i32 = -1;

            // Check if there is a digit word in the line
            for (idx, digit) in digits.iter().enumerate() {
                if line[start_idx..].starts_with(digit) {
                    num = idx as i32 + 1;
                }
            }

            // Check if there is a digit in the line
            if line.chars().nth(start_idx).unwrap().is_digit(10) {
                num = line.chars().nth(start_idx).unwrap().to_digit(10).unwrap() as i32;
            }

            // If there is a digit, store it
            if num != -1 {
                if first_number == 0 {
                    first_number = num;
                    last_number = num;
                } else {
                    last_number = num;
                }
            }
        }

        sum += first_number * 10 + last_number;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn test_solution() {
        let input = include_str!("./inputs/input2_test.txt");
        let output = solution(input).to_string();
        let expected_output = include_str!("./outputs/output2_test.txt");

        assert_eq!(output, expected_output);
    }
}
