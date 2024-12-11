use std::collections::HashMap;

fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

fn blink(stones_count: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones_count: HashMap<u64, u64> = HashMap::default();
    stones_count.iter().for_each(|(stone, count)| {
        if *stone == 0 {
            new_stones_count
                .entry(1)
                .and_modify(|v| *v += count)
                .or_insert(*count);
            return;
        }

        if stone.to_string().len() % 2 == 0 {
            let left_half = stone
                .to_string()
                .chars()
                .take(stone.to_string().len() / 2)
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            let right_half = stone
                .to_string()
                .chars()
                .skip(stone.to_string().len() / 2)
                .collect::<String>()
                .parse::<u64>()
                .unwrap();

            new_stones_count
                .entry(left_half)
                .and_modify(|v| *v += count)
                .or_insert(*count);
            new_stones_count
                .entry(right_half)
                .and_modify(|v| *v += count)
                .or_insert(*count);

            return;
        }

        new_stones_count
            .entry(*stone * 2024)
            .and_modify(|v| *v += count)
            .or_insert(*count);
    });

    new_stones_count
}

fn solution(input: &str) -> String {
    let stones = input.lines().next().map(|line| {
        line.split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    });

    let stones_count: HashMap<u64, u64> =
        stones
            .unwrap()
            .iter()
            .fold(HashMap::default(), |mut acc, &stone| {
                acc.entry(stone).and_modify(|v| *v += 1).or_insert(1);
                acc
            });

    std::iter::successors(Some(stones_count), |stones_count| {
        Some(blink(stones_count.clone()))
    })
    .take(76)
    .last()
    .unwrap()
    .values()
    .sum::<u64>()
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
