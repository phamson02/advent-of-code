use pathfinding::prelude::dijkstra;

fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

struct Puzzle {
    map: Vec<Vec<char>>,
    start: Pos,
    end: Pos,
}

impl Puzzle {
    fn new(input: &str, range: i32, n_corrupted: i32) -> Self {
        let mut map = (0..range)
            .map(|_| vec!['.'; range as usize])
            .collect::<Vec<_>>();

        input
            .lines()
            .take(n_corrupted as usize)
            .map(|line| {
                let parts = line.split(',').collect::<Vec<_>>();
                let x = parts[0].parse::<i32>().unwrap();
                let y = parts[1].parse::<i32>().unwrap();
                (x, y)
            })
            .for_each(|(x, y)| {
                map[x as usize][y as usize] = '#';
            });

        let start = Pos(0, 0);
        let end = Pos(range - 1, range - 1);

        Self { map, start, end }
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

impl Pos {
    fn successors(&self, puzzle: &Puzzle) -> Vec<(Pos, usize)> {
        let Pos(x, y) = self;
        let neighbors = vec![
            (0, -1), // left
            (0, 1),  // right
            (-1, 0), // up
            (1, 0),  // down
        ];

        let mut next_nodes = Vec::new();
        for (dx, dy) in neighbors {
            let new_x = x + dx;
            let new_y = y + dy;
            if puzzle.is_walkable(new_x, new_y) {
                next_nodes.push((Pos(new_x, new_y), 1));
            }
        }

        next_nodes
    }
}

fn solution(input: &str) -> String {
    let puzzle = Puzzle::new(input, 7, 12);

    let start = puzzle.start;
    let end = puzzle.end;

    let result =
        dijkstra(&start, |pos| pos.successors(&puzzle), |pos| *pos == end).expect("No path found");

    result.1.to_string()
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
