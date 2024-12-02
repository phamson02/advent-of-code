fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let levels = line
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let is_ascending = levels[0] < levels[1];
            let mut is_safe = true;
            for i in 0..levels.len() - 1 {
                let diff = (levels[i] - levels[i + 1]).abs();
                if diff < 1 || diff > 3 {
                    is_safe = false;
                    break;
                }
                if levels[i] > levels[i + 1] {
                    if is_ascending {
                        is_safe = false;
                        break;
                    }
                } else {
                    if !is_ascending {
                        is_safe = false;
                        break;
                    }
                }
            }
            is_safe
        })
        .filter(|x| *x)
        .count()
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
