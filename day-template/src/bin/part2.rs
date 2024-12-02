fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn test_solution() {
        let input = include_str!("./tests/input2.txt");
        let output = solution(input).to_string();
        let expected_output = include_str!("./tests/output2.txt");

        assert_eq!(output, expected_output);
    }
}
