fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

struct Matrix {
    data: Vec<Vec<char>>,
    length: usize,
    width: usize,
}

impl Matrix {
    fn new(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let length = data.len();
        let width = data[0].len();

        Self {
            data,
            length,
            width,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.data.get(y).and_then(|row| row.get(x))
    }

    fn get_adjacent(&self, x: usize, y: usize) -> Vec<&char> {
        let mut adjacent = Vec::new();

        if x > 0 {
            adjacent.push(self.get(x - 1, y).unwrap());
        }

        if x < self.width - 1 {
            adjacent.push(self.get(x + 1, y).unwrap());
        }

        if y > 0 {
            adjacent.push(self.get(x, y - 1).unwrap());
        }

        if y < self.length - 1 {
            adjacent.push(self.get(x, y + 1).unwrap());
        }

        if x > 0 && y > 0 {
            adjacent.push(self.get(x - 1, y - 1).unwrap());
        }

        if x > 0 && y < self.length - 1 {
            adjacent.push(self.get(x - 1, y + 1).unwrap());
        }

        if x < self.width - 1 && y > 0 {
            adjacent.push(self.get(x + 1, y - 1).unwrap());
        }

        if x < self.width - 1 && y < self.length - 1 {
            adjacent.push(self.get(x + 1, y + 1).unwrap());
        }

        println!("{:?}", adjacent);

        adjacent
    }
}

fn contains_symbol(adjacent: &[&char]) -> bool {
    adjacent.iter().any(|&c| c != &'.' && !c.is_ascii_digit())
}

fn solution(input: &str) -> u32 {
    let matrix = Matrix::new(input);

    let output: u32 = matrix
        .data
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let mut sum = 0;
            let mut num = 0;
            let mut coors = Vec::new();

            row.iter().enumerate().for_each(|(x, &c)| {
                if c.is_digit(10) {
                    num = num * 10 + c.to_digit(10).unwrap();
                    coors.push((x, y));
                }

                if !c.is_digit(10) || x == row.len() - 1 {
                    let is_part_num = coors
                        .iter()
                        .any(|&(x, y)| contains_symbol(&matrix.get_adjacent(x, y)));

                    if is_part_num {
                        sum += num;
                    }

                    num = 0;
                    coors.clear();
                }
            });
            sum
        })
        .sum();

    output
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
