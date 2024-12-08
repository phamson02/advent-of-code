fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

struct Puzzle {
    puzzle: Vec<Vec<char>>,
    x: i32,
    y: i32,
    max_x: i32,
    max_y: i32,
}

impl Puzzle {
    fn new(puzzle: Vec<Vec<char>>) -> Self {
        let max_x = puzzle.len() as i32;
        let max_y = puzzle[0].len() as i32;

        Self {
            puzzle,
            x: 0,
            y: 0,
            max_x,
            max_y,
        }
    }
}

impl Iterator for Puzzle {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut messages = (String::new(), String::new());

            for i in -1..2 {
                if (self.x + i < 0)
                    || (self.x + i >= self.max_x)
                    || (self.y + i < 0)
                    || (self.y + i >= self.max_y)
                {
                    break;
                }

                messages
                    .0
                    .push(self.puzzle[(self.x + i) as usize][(self.y + i) as usize]);
            }

            for i in -1..2 {
                if (self.x - i < 0)
                    || (self.x - i >= self.max_x)
                    || (self.y + i < 0)
                    || (self.y + i >= self.max_y)
                {
                    break;
                }

                messages
                    .1
                    .push(self.puzzle[(self.x - i) as usize][(self.y + i) as usize]);
            }

            if self.x < self.max_x - 1 {
                self.x += 1;
            } else if self.x == self.max_x - 1 {
                self.x = 0;
                self.y += 1;
            } else if self.y < self.max_y - 1 {
                self.y += 1;
            } else if self.y == self.max_y - 1 {
                self.y = 0;
                self.x += 1;
            }

            if self.y >= self.max_y {
                return None;
            }

            if (messages.0.len() == 3) && (messages.1.len() == 3) {
                return Some(messages);
            }
        }
    }
}

fn solution(input: &str) -> String {
    let puzzle: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let puzzle = Puzzle::new(puzzle);

    let mut count = 0;
    for message in puzzle {
        if (message.0 == "SAM" || message.0 == "MAS") && (message.1 == "SAM" || message.1 == "MAS")
        {
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
        let input = include_str!("./tests/input2.txt");
        let output = solution(input).to_string();
        let expected_output = include_str!("./tests/output2.txt");

        assert_eq!(output, expected_output);
    }
}
