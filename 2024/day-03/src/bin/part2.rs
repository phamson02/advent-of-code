use regex::Regex;

fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> String {
    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\))").unwrap();
    let mut do_mul = true;
    let sum = re
        .find_iter(input)
        .map(|m| {
            if m.as_str() == "do()" {
                do_mul = true;
                0
            } else if m.as_str() == "don't()" {
                do_mul = false;
                0
            } else {
                if !do_mul {
                    0
                } else {
                    let mut mul = m.as_str()[4..m.as_str().len() - 1].split(",");
                    let a = mul.next().unwrap().parse::<i32>().unwrap();
                    let b = mul.next().unwrap().parse::<i32>().unwrap();
                    a * b
                }
            }
        })
        .sum::<i32>();
    sum.to_string()
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
