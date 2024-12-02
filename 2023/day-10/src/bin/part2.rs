use queues::*;

fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(PartialEq, Debug, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
}

#[derive(PartialEq, Debug, Clone)]
enum Tile {
    Pipe(Pipe),
    NonPipe,
    Start,
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<(Tile, (usize, usize))>>,
}

impl Grid {
    fn new(grid_data: Vec<Vec<(Tile, (usize, usize))>>, start_coors: (usize, usize)) -> Self {
        let mut grid = Self { grid: grid_data };

        let start_pipe = get_start_pipe(&grid, start_coors.0, start_coors.1).unwrap();

        grid.grid[start_coors.1][start_coors.0] = (Tile::Pipe(start_pipe), start_coors);

        grid
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        if let Some((tile, _)) = self.grid.get(y)?.get(x) {
            return Some(tile);
        }

        None
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(&Tile, (usize, usize))> {
        let mut neighbors = Vec::new();

        if y > 0 {
            if let Some(tile) = self.get(x, y - 1) {
                neighbors.push((tile, (x, y - 1)));
            }
        }

        if let Some(tile) = self.get(x, y + 1) {
            neighbors.push((tile, (x, y + 1)));
        }

        if x > 0 {
            if let Some(tile) = self.get(x - 1, y) {
                neighbors.push((tile, (x - 1, y)));
            }
        }

        if let Some(tile) = self.get(x + 1, y) {
            neighbors.push((tile, (x + 1, y)));
        }

        neighbors
    }

    fn is_connected(&self, coors1: (usize, usize), coors2: (usize, usize)) -> bool {
        use self::Pipe::*;
        use Tile::*;

        let (x1, y1) = coors1;
        let (x2, y2) = coors2;
        let tile1 = self.get(x1, y1).unwrap();
        let tile2 = self.get(x2, y2).unwrap();

        match (tile1, tile2) {
            (Pipe(Vertical), Pipe(Vertical)) => x1 == x2, // | and |
            (Pipe(Vertical), Pipe(BendNE)) => x1 == x2 && y1 < y2, // | and L
            (Pipe(Vertical), Pipe(BendNW)) => x1 == x2 && y1 < y2, // | and J
            (Pipe(Vertical), Pipe(BendSW)) => x1 == x2 && y1 > y2, // | and 7
            (Pipe(Vertical), Pipe(BendSE)) => x1 == x2 && y1 > y2, // | and F
            (Pipe(Horizontal), Pipe(Horizontal)) => y1 == y2,
            (Pipe(Horizontal), Pipe(BendNE)) => y1 == y2 && x1 > x2, // - and L
            (Pipe(Horizontal), Pipe(BendNW)) => y1 == y2 && x1 < x2, // - and J
            (Pipe(Horizontal), Pipe(BendSW)) => y1 == y2 && x1 < x2, // - and 7
            (Pipe(Horizontal), Pipe(BendSE)) => y1 == y2 && x1 > x2, // - and F
            (Pipe(BendNE), Pipe(BendSW)) => (x1 == x2 && y1 > y2) || (y1 == y2 && x1 < x2), // L and 7
            (Pipe(BendNE), Pipe(BendSE)) => (x1 == x2 && y1 > y2) || (y1 == y2 && x1 > x2), // L and F
            (Pipe(BendNE), Pipe(BendNW)) => y1 == y2 && x1 < x2, // L and J
            (Pipe(BendNW), Pipe(BendSW)) => (x1 == x2 && y1 > y2) || (y1 == y2 && x1 < x2), // J and 7
            (Pipe(BendNW), Pipe(BendSE)) => (x1 == x2 && y1 > y2) || (y1 == y2 && x1 > x2), // J and F
            (Pipe(BendSW), Pipe(BendSE)) => y1 == y2 && x1 > x2, // 7 and F
            _ => false,
        }
    }
}

fn parse_pipe(char: char) -> Tile {
    use self::Pipe::*;
    use Tile::*;

    match char {
        '|' => Pipe(Vertical),
        '-' => Pipe(Horizontal),
        'L' => Pipe(BendNE),
        'J' => Pipe(BendNW),
        '7' => Pipe(BendSW),
        'F' => Pipe(BendSE),
        'S' => Start,
        '.' => NonPipe,
        _ => panic!("Invalid character"),
    }
}

fn get_start_pipe(grid: &Grid, x: usize, y: usize) -> Result<Pipe, &'static str> {
    use self::Pipe::*;

    let mut cloned_grid = grid.clone();
    let neighbors = grid.get_neighbors(x, y);

    for pipe in vec![Vertical, Horizontal, BendNE, BendNW, BendSW, BendSE] {
        let mut connected = 0;

        cloned_grid.grid[y][x] = (Tile::Pipe(pipe.clone()), (x, y));

        // Check if the pipe is connected to the two neighbors
        for (_, coors) in &neighbors {
            if cloned_grid.is_connected((x, y), *coors) || cloned_grid.is_connected(*coors, (x, y))
            {
                connected += 1;
            }
        }

        if connected == 2 {
            println!("Found start pipe: {:?}", pipe);
            return Ok(pipe);
        }
    }

    Err("No suitable start pipe found")
}

fn solution(input: &str) -> String {
    let mut start_coors: (usize, usize) = (0, 0);

    let grid_data: Vec<Vec<(Tile, (usize, usize))>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    let tile = parse_pipe(char);

                    if tile == Tile::Start {
                        start_coors = (x, y);
                    }

                    (tile, (x, y))
                })
                .collect()
        })
        .collect();

    let grid = Grid::new(grid_data, start_coors);

    let mut visited = Vec::new();
    let mut frontiers: Queue<((usize, usize), usize)> = queue![];

    frontiers.add((start_coors, 0)).unwrap();
    visited.push((start_coors, 0));

    while let Ok((coors, distance)) = frontiers.remove() {
        let neighbors = grid.get_neighbors(coors.0, coors.1);
        let next_distance = distance + 1;

        for (neighbor, next_coors) in neighbors {
            if neighbor == &Tile::NonPipe {
                continue;
            }

            if visited.iter().any(|(c, _)| c == &next_coors) {
                continue;
            }

            if !grid.is_connected(coors, next_coors) && !grid.is_connected(next_coors, coors) {
                continue;
            }

            frontiers.add((next_coors, next_distance)).unwrap();
            visited.push((next_coors, next_distance));
        }
    }

    visited
        .iter()
        .max_by_key(|(_, distance)| distance)
        .unwrap()
        .1
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
