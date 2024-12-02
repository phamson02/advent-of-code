use core::panic;

fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

struct Pattern {
    data: Vec<Vec<bool>>,
    transpose_data: Vec<Vec<bool>>,
}

impl Pattern {
    fn new(data: Vec<Vec<bool>>) -> Self {
        let transpose_data = data[0]
            .iter()
            .enumerate()
            .map(|(i, _)| data.iter().map(|row| row[i]).collect())
            .collect::<Vec<Vec<bool>>>();

        Self {
            data,
            transpose_data,
        }
    }

    fn _find_reflection_line(data: Vec<Vec<bool>>) -> Option<usize> {
        let start_line = data[0].clone();

        let reflect_idx = data
            .iter()
            .enumerate()
            .skip(1)
            .rev()
            .find(|(_, line)| **line == start_line)
            .map(|(idx, _)| idx);

        if reflect_idx.is_none() {
            let start_line = data.last().unwrap().clone();

            let reflect_idx = data
                .iter()
                .enumerate()
                .rev()
                .skip(1)
                .rev()
                .find(|(_, line)| **line == start_line)
                .map(|(idx, _)| idx);

            if reflect_idx.is_none() {
                return None;
            } else {
                return Some((data.len() - reflect_idx.unwrap()) / 2 + reflect_idx.unwrap());
            }
        }

        Some((reflect_idx.unwrap() + 1) / 2)
    }

    fn _find_reflection(data: Vec<Vec<bool>>) -> Option<usize> {
        let reflect_idx = Self::_find_reflection_line(data.clone());

        if reflect_idx.is_none() {
            return None;
        }

        let reflect_idx = reflect_idx.unwrap();

        // Check if the reflection is valid
        let mut valid = true;

        for i in 0..reflect_idx {
            if reflect_idx * 2 - i - 1 >= data.len() {
                continue;
            } else {
                if data[i] != data[reflect_idx * 2 - i - 1] {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            return Some(reflect_idx);
        } else {
            return Some(0);
        }
    }

    fn get_reflection(&self) -> Option<Reflection> {
        if let Some(i) = Self::_find_reflection(self.data.clone()) {
            return Some(Reflection::Horizontal(i));
        }

        if let Some(i) = Self::_find_reflection(self.transpose_data.clone()) {
            return Some(Reflection::Vertical(i));
        }

        None
    }
}

fn solution(input: &str) -> String {
    let patterns_data = input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => panic!("Invalid character"),
                        })
                        .collect::<Vec<bool>>()
                })
                .collect::<Vec<Vec<bool>>>()
        })
        .collect::<Vec<Vec<Vec<bool>>>>();

    let patterns = patterns_data
        .iter()
        .map(|data| Pattern::new(data.clone()))
        .collect::<Vec<Pattern>>();

    patterns
        .iter()
        .map(|pattern| {
            if let Some(reflection) = pattern.get_reflection() {
                dbg!(&reflection);

                match reflection {
                    Reflection::Horizontal(i) => i * 100,
                    Reflection::Vertical(i) => i,
                }
            } else {
                panic!("No reflection found");
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
