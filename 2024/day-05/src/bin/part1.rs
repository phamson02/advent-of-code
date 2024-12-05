fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> String {
    let parts = input.split_once("\n\n").unwrap();

    let rules: Vec<(i32, i32)> = parts
        .0
        .lines()
        .map(|line| {
            line.split_once("|")
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect();
    let updates: Vec<Vec<i32>> = parts
        .1
        .lines()
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    updates
        .iter()
        .filter(|update| {
            let mut is_correct = true;
            for (a, b) in &rules {
                if !update.contains(a) || !update.contains(b) {
                    continue;
                }
                if update.iter().position(|&x| x == *a).unwrap()
                    > update.iter().position(|&x| x == *b).unwrap()
                {
                    is_correct = false;
                    break;
                }
            }
            is_correct
        })
        .map(|update|
        // Return the middle element
        update[update.len() / 2])
        .sum::<i32>()
        .to_string()
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
