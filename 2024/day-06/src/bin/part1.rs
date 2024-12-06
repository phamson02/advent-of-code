fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn is_outside(puzzle: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    x >= (puzzle[0].len() as i32) || x < 0 || y >= (puzzle.len() as i32) || y < 0
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn get_new_pos(pos: (i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

fn solution(input: &str) -> String {
    let mut puzzle: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut guard_pos: (i32, i32) = puzzle
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '^').map(|x| (x, y)))
        .map(|(x, y)| (x as i32, y as i32))
        .unwrap();

    puzzle[guard_pos.1 as usize][guard_pos.0 as usize] = 'X';

    let mut direction = Direction::Up;

    loop {
        let new_pos = get_new_pos(guard_pos, &direction);
        // if new pos is inside the puzzle and is a wall
        if !is_outside(&puzzle, new_pos.0, new_pos.1) {
            if puzzle[new_pos.1 as usize][new_pos.0 as usize] == '#' {
                // turn right
                direction = direction.turn_right();
            } else {
                // can move forward
                guard_pos = new_pos;
                puzzle[guard_pos.1 as usize][guard_pos.0 as usize] = 'X';
            }
        } else {
            break;
        }
    }

    puzzle
        .iter()
        .map(|row| row.iter().filter(|&&c| c == 'X').count())
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
