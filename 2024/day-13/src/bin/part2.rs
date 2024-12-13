use itertools::Itertools;
use regex::Regex;
fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

fn solve_linear_equations(
    a1: f64,
    b1: f64,
    c1: f64,
    a2: f64,
    b2: f64,
    c2: f64,
) -> Option<(f64, f64)> {
    let determinant = a1 * b2 - a2 * b1;
    if determinant == 0.0 {
        None // No unique solution exists
    } else {
        let x = (c1 * b2 - c2 * b1) / determinant;
        let y = (a1 * c2 - a2 * c1) / determinant;
        Some((x, y))
    }
}

#[derive(Debug)]
struct Puzzle {
    a1: isize,
    b1: isize,
    c1: isize,
    a2: isize,
    b2: isize,
    c2: isize,
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
                .collect::<Vec<(isize, isize)>>();

            let target = re
                .captures(lines[2])
                .map(|cap| {
                    (
                        cap[1].parse::<isize>().unwrap(),
                        cap[2].parse::<isize>().unwrap(),
                    )
                })
                .unwrap();

            let target = (target.0 + 10000000000000, target.1 + 10000000000000);

            Puzzle {
                a1: buttons[0].0,
                b1: buttons[1].0,
                c1: target.0,
                a2: buttons[0].1,
                b2: buttons[1].1,
                c2: target.1,
            }
        })
        .collect::<Vec<Puzzle>>();

    puzzles
        .iter()
        .map(|p| {
            match solve_linear_equations(
                p.a1 as f64,
                p.b1 as f64,
                p.c1 as f64,
                p.a2 as f64,
                p.b2 as f64,
                p.c2 as f64,
            ) {
                Some((x, y)) => {
                    // Check if x and y is integer
                    if x.fract() != 0.0 || y.fract() != 0.0 {
                        return 0;
                    }

                    let x = x as isize;
                    let y = y as isize;

                    x * 3 + y
                }
                None => 0,
            }
        })
        .sum::<isize>()
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
