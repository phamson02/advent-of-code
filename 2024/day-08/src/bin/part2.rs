use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

fn get_antinodes(antenna_coors: Vec<(i32, i32)>, puzzle_size: (i32, i32)) -> Vec<(i32, i32)> {
    // go through every pair of antennas
    let mut antinodes = HashSet::new();
    for (i, a1) in antenna_coors.iter().enumerate() {
        for a2 in antenna_coors.iter().skip(i + 1) {
            let (x1, y1) = a1;
            let (x2, y2) = a2;

            // get the slope of the line
            let x_diff = x2 - x1;
            let y_diff = y2 - y1;

            let mut k = 0;
            loop {
                let x = x1 - k * x_diff;
                let y = y1 - k * y_diff;

                if (x >= 0) && (x < puzzle_size.0) && (y >= 0) && (y < puzzle_size.1) {
                    antinodes.insert((x, y));
                    k += 1;
                } else {
                    break;
                }
            }

            let mut k = 0;
            loop {
                let x = x2 + k * x_diff;
                let y = y2 + k * y_diff;

                if (x >= 0) && (x < puzzle_size.0) && (y >= 0) && (y < puzzle_size.1) {
                    antinodes.insert((x, y));
                    k += 1;
                } else {
                    break;
                }
            }
        }
    }

    antinodes.into_iter().collect()
}

fn solution(input: &str) -> String {
    let puzzle: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut antennas_coors: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    puzzle.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if c != &'.' {
                antennas_coors
                    .entry(*c)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));
            }
        });
    });

    let all_antinodes: HashSet<(i32, i32)> =
        antennas_coors.iter().fold(HashSet::new(), |acc, (_, v)| {
            let antinodes = get_antinodes(v.clone(), (puzzle[0].len() as i32, puzzle.len() as i32));
            acc.union(&antinodes.into_iter().collect())
                .cloned()
                .collect()
        });

    all_antinodes.len().to_string()
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
