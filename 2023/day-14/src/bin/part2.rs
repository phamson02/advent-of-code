fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

enum Direction {
    North,
    West,
    South,
    East,
}

fn tilt(columns: &Vec<Vec<char>>, direction: Direction) -> Vec<Vec<char>> {
    let mut new_columns = columns.clone();

    new_columns = match direction {
        Direction::North => new_columns,
        Direction::West => {
            // transpose data
            new_columns
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    c.iter()
                        .enumerate()
                        .map(|(j, _)| new_columns[j][i])
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>()
        }
        Direction::South => {
            // reverse data
            let mut reversed_columns = new_columns.clone();
            for (i, c) in new_columns.iter().enumerate() {
                reversed_columns[i] = c.iter().rev().cloned().collect::<Vec<char>>();
            }
            reversed_columns
        }
        Direction::East => {
            // rotate data counter-clockwise
            let transposed_columns = new_columns
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    c.iter()
                        .enumerate()
                        .map(|(j, _)| new_columns[j][i])
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>();

            transposed_columns
                .iter()
                .map(|c| c.iter().rev().cloned().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        }
    };

    let new_columns = new_columns
        .iter()
        .map(|column| {
            let mut mut_column = column.clone();
            let mut start = 0;
            let mut is_in = false;
            for (i, c) in column.iter().enumerate() {
                if *c == '#' {
                    start = i + 1;
                    is_in = true;
                } else if *c == 'O' {
                    if is_in {
                        let end = i;
                        let mut is_swapped = false;
                        for j in (start + 1..=end).rev() {
                            mut_column[j - 1] = mut_column[j].clone();
                            mut_column[j] = '.';
                            is_swapped = true;
                        }

                        if is_swapped {
                            start += 1;
                        } else {
                            start = i + 1;
                        }
                        is_in = true;
                    } else {
                        start = 1;
                        is_in = true;
                        mut_column[i] = '.';
                        mut_column[0] = 'O';
                    }
                }
            }
            mut_column.retain(|c| *c != ' ');
            mut_column
        })
        .collect::<Vec<Vec<char>>>();

    match direction {
        Direction::North => new_columns,
        Direction::West => {
            // transpose data
            new_columns
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    c.iter()
                        .enumerate()
                        .map(|(j, _)| new_columns[j][i])
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>()
        }
        Direction::South => {
            // reverse data
            let mut reversed_columns = new_columns.clone();
            for (i, c) in new_columns.iter().enumerate() {
                reversed_columns[i] = c.iter().rev().cloned().collect::<Vec<char>>();
            }
            reversed_columns
        }
        Direction::East => {
            // rotate data counter-clockwise
            let new_columns = new_columns
                .iter()
                .map(|c| c.iter().rev().cloned().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

            new_columns
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    c.iter()
                        .enumerate()
                        .map(|(j, _)| new_columns[j][i])
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>()
        }
    }
}

fn display(columns: &Vec<Vec<char>>) {
    columns[0].iter().enumerate().for_each(|(i, _)| {
        columns
            .iter()
            .for_each(|column| print!("{}", column[i].to_string()));
        println!();
    });
    println!();
}

fn cycle_transform(columns: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_columns = columns.clone();

    new_columns = tilt(&new_columns, Direction::North);
    new_columns = tilt(&new_columns, Direction::West);
    new_columns = tilt(&new_columns, Direction::South);
    new_columns = tilt(&new_columns, Direction::East);

    new_columns
}

fn solution(input: &str) -> String {
    let columns = input
        .lines()
        .next()
        .expect("No lines in input")
        .chars()
        .enumerate()
        .map(|(i, _)| {
            input
                .lines()
                .map(|line| line.chars().nth(i).expect("No char at index"))
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    display(&columns);

    let mut mut_columns: Vec<Vec<char>> = columns.clone();

    // let new_columns = cycle_transform(&mut_columns);

    // Check when the columns pattern repeats after cycle_transform
    let mut cycle = 0;
    loop {
        mut_columns = cycle_transform(&mut_columns);
        cycle += 1;
        if mut_columns
            .iter()
            .enumerate()
            .all(|(i, c)| c.iter().enumerate().all(|(j, c)| c == &columns[i][j]))
            && cycle > 1
        {
            break;
        }
        if cycle < 4 {
            display(&mut_columns);
        }
        // print!("Cycle: {}\r", cycle)
    }

    println!("Cycle: {}", cycle);

    mut_columns
        .iter()
        .map(|column| {
            column
                .iter()
                .enumerate()
                .map(|(i, c)| if c == &'O' { columns[0].len() - i } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
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
