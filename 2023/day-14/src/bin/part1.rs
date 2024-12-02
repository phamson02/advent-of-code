fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
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

    // Remove all . chars that is between # chars and O chars
    let mut_columns = columns
        .iter()
        .map(|column| {
            dbg!(&column);
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
                        println!("Start: {}, End: {}", start, end);
                        for j in (start + 1..=end).rev() {
                            println!("Swapping {} and {}", j - 1, j);
                            mut_column[j - 1] = mut_column[j].clone();
                            mut_column[j] = '.';
                            dbg!(&mut_column);
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
        let input = include_str!("./tests/input1.txt");
        let output = solution(input).to_string();
        let expected_output = include_str!("./tests/output1.txt");

        assert_eq!(output, expected_output);
    }
}
