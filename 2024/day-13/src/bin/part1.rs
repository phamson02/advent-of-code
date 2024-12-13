use itertools::Itertools;
use regex::Regex;
use std::cmp::min;

fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn get_min_tokens_from_buttons(
    index: usize,
    buttons: &Vec<(isize, isize)>,
    target: (isize, isize),
    usage: &mut Vec<usize>,
    current_usage: &mut Vec<usize>,
) -> isize {
    if target.0 == 0 && target.1 == 0 {
        *usage = current_usage.clone();
        return 0;
    }

    if target.0 < 0 || target.1 < 0 || index >= buttons.len() {
        return isize::MAX;
    }

    // Try taking the current button
    current_usage[index] += 1;
    let mut take = get_min_tokens_from_buttons(
        index,
        buttons,
        (target.0 - buttons[index].0, target.1 - buttons[index].1),
        usage,
        current_usage,
    );
    current_usage[index] -= 1; // Undo the choice

    if take != isize::MAX {
        take += 1;
    }

    // Try skipping the current button
    let leave = get_min_tokens_from_buttons(index + 1, buttons, target, usage, current_usage);

    min(take, leave)
}

fn get_min_tokens(buttons: &Vec<(isize, isize)>, target: (isize, isize)) -> (isize, Vec<usize>) {
    let mut usage = vec![0; buttons.len()];
    let mut current_usage = vec![0; buttons.len()];

    let min_tokens =
        get_min_tokens_from_buttons(0, buttons, target, &mut usage, &mut current_usage);

    if min_tokens == isize::MAX {
        (min_tokens, vec![]) // Return an empty vector if target can't be reached
    } else {
        (min_tokens, usage)
    }
}

struct Puzzle {
    buttons: Vec<(isize, isize)>,
    target: (isize, isize),
}

fn solution(input: &str) -> String {
    let re = Regex::new(r"X[\+=](\d+),?\s*Y[\+=](\d+)").unwrap();
    let puzzles = input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            let lines = chunk.collect::<Vec<&str>>();
            let buttons = (0..=1)
                .map(|i| {
                    re.captures(lines[i])
                        .map(|cap| {
                            (
                                cap[1].parse::<isize>().unwrap(),
                                cap[2].parse::<isize>().unwrap(),
                            )
                        })
                        .unwrap()
                })
                .collect();

            let target = re
                .captures(lines[2])
                .map(|cap| {
                    (
                        cap[1].parse::<isize>().unwrap(),
                        cap[2].parse::<isize>().unwrap(),
                    )
                })
                .unwrap();

            Puzzle { buttons, target }
        })
        .collect::<Vec<Puzzle>>();

    puzzles
        .iter()
        .map(|puzzle| {
            let (_, buttons_pressed) = get_min_tokens(&puzzle.buttons, puzzle.target);
            dbg!(&buttons_pressed);

            if buttons_pressed.is_empty() {
                0
            } else {
                buttons_pressed[0] * 3 + buttons_pressed[1] * 1
            }
        })
        .sum::<usize>()
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
