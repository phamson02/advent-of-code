fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    ColumnForward,
    ColumnBackward,
    RowForward,
    RowBackward,
    DiagonalForward,
    DiagonalBackward,
    OtherDiagonalForward,
    OtherDiagonalBackward,
}

struct Puzzle {
    puzzle: Vec<Vec<char>>,
    x: usize,
    y: usize,
    curr_direction: Direction,
}

impl Iterator for Puzzle {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut message = String::from(self.puzzle[self.x][self.y]);
            match self.curr_direction {
                Direction::ColumnForward => {
                    for i in 1..4 {
                        if self.x + i >= self.puzzle.len() {
                            break;
                        }
                        message.push(self.puzzle[self.x + i][self.y]);
                    }
                }
                Direction::ColumnBackward => {
                    for i in 1..4 {
                        if self.x < i {
                            break;
                        }
                        message.push(self.puzzle[self.x - i][self.y]);
                    }
                }
                Direction::RowForward => {
                    for i in 1..4 {
                        if self.y + i >= self.puzzle[0].len() {
                            break;
                        }
                        message.push(self.puzzle[self.x][self.y + i]);
                    }
                }
                Direction::RowBackward => {
                    for i in 1..4 {
                        if self.y < i {
                            break;
                        }
                        message.push(self.puzzle[self.x][self.y - i]);
                    }
                }
                Direction::DiagonalForward => {
                    for i in 1..4 {
                        if self.x + i >= self.puzzle.len() || self.y + i >= self.puzzle[0].len() {
                            break;
                        }
                        message.push(self.puzzle[self.x + i][self.y + i]);
                    }
                }
                Direction::DiagonalBackward => {
                    for i in 1..4 {
                        if self.x < i || self.y < i {
                            break;
                        }
                        message.push(self.puzzle[self.x - i][self.y - i]);
                    }
                }
                Direction::OtherDiagonalForward => {
                    for i in 1..4 {
                        if self.x + i >= self.puzzle.len() || self.y < i {
                            break;
                        }
                        message.push(self.puzzle[self.x + i][self.y - i]);
                    }
                }
                Direction::OtherDiagonalBackward => {
                    for i in 1..4 {
                        if self.x < i || self.y + i >= self.puzzle[0].len() {
                            break;
                        }
                        message.push(self.puzzle[self.x - i][self.y + i]);
                    }
                }
            };

            if self.curr_direction == Direction::ColumnForward {
                if self.x < self.puzzle.len() - 1 {
                    self.x += 1;
                } else if self.x == self.puzzle.len() - 1 {
                    self.x = 0;
                    self.y += 1;
                } else if self.y < self.puzzle[0].len() - 1 {
                    self.y += 1;
                } else if self.y == self.puzzle[0].len() - 1 {
                    self.y = 0;
                    self.x += 1;
                }

                if self.y >= self.puzzle[0].len() {
                    return None;
                }
            }

            self.curr_direction = match self.curr_direction {
                Direction::ColumnForward => Direction::ColumnBackward,
                Direction::ColumnBackward => Direction::RowForward,
                Direction::RowForward => Direction::RowBackward,
                Direction::RowBackward => Direction::DiagonalForward,
                Direction::DiagonalForward => Direction::DiagonalBackward,
                Direction::DiagonalBackward => Direction::OtherDiagonalForward,
                Direction::OtherDiagonalForward => Direction::OtherDiagonalBackward,
                Direction::OtherDiagonalBackward => Direction::ColumnForward,
            };

            if message.len() == 4 {
                return Some(message.chars().collect());
            }
        }
    }
}

fn solution(input: &str) -> String {
    let puzzle: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let puzzle = Puzzle {
        puzzle,
        x: 0,
        y: 0,
        curr_direction: Direction::ColumnForward,
    };

    let mut count = 0;
    for message in puzzle {
        if message == "XMAS" {
            count += 1;
        }
    }

    count.to_string()
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
