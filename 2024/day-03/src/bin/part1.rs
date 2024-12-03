use regex::Regex;

fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> String {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let sum = re
        .find_iter(input)
        .map(|m| {
            let mut mul = m.as_str()[4..m.as_str().len() - 1].split(",");
            let a = mul.next().unwrap().parse::<i32>().unwrap();
            let b = mul.next().unwrap().parse::<i32>().unwrap();
            a * b
        })
        .sum::<i32>();
    sum.to_string()
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
