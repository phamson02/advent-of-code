use std::collections::HashMap;

fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> String {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    let mut appears: HashMap<i32, i32> = HashMap::new();
    let mut scores = 0;

    input.lines().for_each(|line| {
        let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();

        first_list.push(parts[0].parse().unwrap());
        second_list.push(parts[1].parse().unwrap());
    });

    first_list
        .iter()
        .for_each(|a| scores += (second_list.iter().filter(|x| **x == *a).count() as i32) * *a);

    scores.to_string()
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
