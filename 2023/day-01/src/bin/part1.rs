fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> i32 {
    let mut sum: i32 = 0;
    for line in input.lines() {
        let mut first_number: i32 = 0;
        let mut last_number: i32 = 0;
        for char in line.chars() {
            if char.is_digit(10) {
                let number = char.to_digit(10).unwrap() as i32;
                if first_number == 0 {
                    first_number = number;
                    last_number = number;
                } else {
                    last_number = number;
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
        let input = include_str!("./inputs/input1_test.txt");
        let output = solution(input).to_string();
        let expected_output = include_str!("./outputs/output1_test.txt");

        assert_eq!(output, expected_output);
    }
}
