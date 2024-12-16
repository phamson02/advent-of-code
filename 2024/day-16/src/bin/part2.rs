use std::collections::HashSet;

use pathfinding::prelude::yen;

fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(PartialEq, Clone, Debug, Eq, Hash, Ord, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

struct Puzzle {
    map: Vec<Vec<char>>,
    start: Pos,
    end: Pos,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);

        let map = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        'S' => {
                            start_pos = (i as i32, j as i32);
                            c
                        }
                        'E' => {
                            end_pos = (i as i32, j as i32);
                            c
                        }
                        _ => c,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            map,
            start: Pos(start_pos.0, start_pos.1),
            end: Pos(end_pos.0, end_pos.1),
        }
    }

    fn width(&self) -> i32 {
        self.map[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.map.len() as i32
    }

    fn is_walkable(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.height() || y < 0 || y >= self.width() {
            return false;
        }
        self.map[x as usize][y as usize] != '#'
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    pos: Pos,
    dir: Direction,
}

fn successors(node: &Node, puzzle: &Puzzle) -> Vec<(Node, usize)> {
    let Pos(x, y) = node.pos;

    let neighbors: Vec<(Direction, i32, i32)> = match node.dir {
        Direction::North => vec![
            (Direction::North, -1, 0),
            (Direction::East, 0, 1),
            (Direction::West, 0, -1),
        ],
        Direction::East => vec![
            (Direction::East, 0, 1),
            (Direction::North, -1, 0),
            (Direction::South, 1, 0),
        ],
        Direction::South => vec![
            (Direction::South, 1, 0),
            (Direction::East, 0, 1),
            (Direction::West, 0, -1),
        ],
        Direction::West => vec![
            (Direction::West, 0, -1),
            (Direction::North, -1, 0),
            (Direction::South, 1, 0),
        ],
    };

    let mut next_nodes = Vec::new();
    for (new_dir, dx, dy) in neighbors {
        let new_x = x + dx;
        let new_y = y + dy;
        if puzzle.is_walkable(new_x, new_y) {
            // If turning to a new direction, cost = 1000, else cost = 1
            let cost = if new_dir == node.dir { 1 } else { 1001 };
            next_nodes.push((
                Node {
                    pos: Pos(new_x, new_y),
                    dir: new_dir,
                },
                cost,
            ));
        }
    }

    next_nodes
}

fn solution(input: &str) -> String {
    let puzzle = Puzzle::new(input);
    // Start node with an initial direction, say East.
    let start_node = Node {
        pos: puzzle.start.clone(),
        dir: Direction::East,
    };

    let end = puzzle.end.clone();

    let result = yen(
        &start_node,
        |node| successors(node, &puzzle),
        |node| node.pos == end,
        20,
    );

    let best_path_cost = result.first().unwrap().1;
    dbg!(best_path_cost);

    result
        .iter()
        .filter(|(_, cost)| *cost == best_path_cost)
        .map(|(nodes, _)| nodes.iter().map(|node| node.pos).collect::<Vec<_>>())
        .flatten()
        .collect::<HashSet<_>>()
        .len()
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
