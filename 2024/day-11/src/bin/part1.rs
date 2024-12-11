fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones: Vec<u64> = Vec::new();

    stones.iter().for_each(|stone| {
        if *stone == 0 {
            new_stones.push(1);
            return;
        }

        if stone.to_string().len() % 2 == 0 {
            let left_half = stone
                .to_string()
                .chars()
                .take(stone.to_string().len() / 2)
                .collect::<String>();
            let right_half = stone
                .to_string()
                .chars()
                .skip(stone.to_string().len() / 2)
                .collect::<String>();

            new_stones.push(left_half.parse::<u64>().unwrap());
            new_stones.push(right_half.parse::<u64>().unwrap());
            return;
        }

        new_stones.push(*stone * 2024);
    });

    new_stones
}

fn solution(input: &str) -> String {
    let stones = input.lines().next().map(|line| {
        line.split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    });

    std::iter::successors(stones, |stones| Some(blink(stones)))
        .take(26)
        .last()
        .unwrap()
        .len()
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
