fn main() {
    let input = include_str!("./inputs/input2.txt");
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

    fn get_adjacent(&self, x: usize, y: usize) -> Vec<((usize, usize), &char)> {
        let mut adjacent = Vec::new();

        if x > 0 {
            adjacent.push(((x - 1, y), self.get(x - 1, y).unwrap()));
        }

        if x < self.width - 1 {
            adjacent.push(((x + 1, y), self.get(x + 1, y).unwrap()));
        }

        if y > 0 {
            adjacent.push(((x, y - 1), self.get(x, y - 1).unwrap()));
        }

        if y < self.length - 1 {
            adjacent.push(((x, y + 1), self.get(x, y + 1).unwrap()));
        }

        if x > 0 && y > 0 {
            adjacent.push(((x - 1, y - 1), self.get(x - 1, y - 1).unwrap()));
        }

        if x > 0 && y < self.length - 1 {
            adjacent.push(((x - 1, y + 1), self.get(x - 1, y + 1).unwrap()));
        }

        if x < self.width - 1 && y > 0 {
            adjacent.push(((x + 1, y - 1), self.get(x + 1, y - 1).unwrap()));
        }

        if x < self.width - 1 && y < self.length - 1 {
            adjacent.push(((x + 1, y + 1), self.get(x + 1, y + 1).unwrap()));
        }

        adjacent
    }
}

#[derive(Debug)]
struct Gear {
    coor: (usize, usize),
    part_numbers: Vec<usize>,
}

// Two gear equals if they have the same coor
impl PartialEq for Gear {
    fn eq(&self, other: &Self) -> bool {
        self.coor == other.coor
    }
}

fn solution(input: &str) -> usize {
    let matrix = Matrix::new(input);

    let mut gears = Vec::new();

    matrix.data.iter().enumerate().for_each(|(y, row)| {
        let mut num = 0;
        let mut coors = Vec::new();

        row.iter().enumerate().for_each(|(x, &c)| {
            if c.is_digit(10) {
                num = num * 10 + c.to_digit(10).unwrap();
                coors.push((x, y));
            }

            if !c.is_digit(10) || x == row.len() - 1 {
                // Get gear coors by finding coors of adjacent cells that are '*'
                let mut gear_coors = Vec::new();

                coors.iter().for_each(|&(x, y)| {
                    let adjacent = matrix.get_adjacent(x, y);

                    adjacent.iter().for_each(|((x_c, y_c), &c)| {
                        if c == '*' {
                            gear_coors.push((*x_c, *y_c));
                        }
                    });
                });

                if gear_coors.len() > 0 {
                    let mut gear = Gear {
                        coor: gear_coors[0],
                        part_numbers: vec![num as usize],
                    };

                    if gears.len() == 0 {
                        gears.push(gear);
                    } else {
                        let mut found = false;

                        gears.iter_mut().for_each(|g: &mut Gear| {
                            if g == &mut gear {
                                g.part_numbers.push(num as usize);
                                found = true;
                            }
                        });

                        if !found {
                            gears.push(gear);
                        }
                    };
                }

                num = 0;
                coors.clear();
            }
        });
    });

    let output = gears
        .iter()
        .filter(|g| g.part_numbers.len() > 1)
        .map(|g| g.part_numbers.iter().product::<usize>())
        .sum::<usize>();

    output
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
