fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

fn check(levels: &mut Vec<i32>) -> bool {
    let is_ascending = levels[0] < levels[1];
    let mut is_safe = true;
    let mut has_torelated = false;
    for i in 0..levels.len() - 1 {
        dbg!(levels[i], levels[i + 1]);
        let diff = (levels[i] - levels[i + 1]).abs();
        if diff < 1 || diff > 3 {
            is_safe = false;
        }
        if levels[i] > levels[i + 1] {
            if is_ascending {
                is_safe = false;
            }
        } else {
            if !is_ascending {
                is_safe = false;
            }
        }
        if !is_safe && !has_torelated {
            if i == levels.len() - 2 {
                dbg!(i, levels[i], levels[i + 1], has_torelated);
                is_safe = true;
                break;
            }
            dbg!(levels[i], levels[i + 1], has_torelated);
            levels[i + 1] = levels[i];
            is_safe = true;
            has_torelated = true;
        } else if !is_safe && has_torelated {
            break;
        } else {
            continue;
        }
    }
    is_safe
}

fn solution(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut levels = line
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let mut reversed = levels.iter().rev().copied().collect::<Vec<i32>>();
            let is_sort = check(&mut levels);
            let is_sort_reverse = check(&mut reversed);

            is_sort || is_sort_reverse
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
        let input = include_str!("./tests/input2.txt");
        let output = solution(input).to_string();
        let expected_output = include_str!("./tests/output2.txt");

        assert_eq!(output, expected_output);
    }
}
