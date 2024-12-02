fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> String {
    // parse two lists of numbers
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    input.lines().for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        first_list.push(parts[0].parse().unwrap());
        second_list.push(parts[1].parse().unwrap());
    });

    // sort the lists
    first_list.sort();
    second_list.sort();

    let sum_diff: i32 = first_list
        .iter()
        .zip(second_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    sum_diff.to_string()
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
