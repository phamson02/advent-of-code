use std::collections::HashSet;

fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn is_in_bounds(grid: &Vec<Vec<usize>>, row: i32, col: i32) -> bool {
    if row < 0 || col < 0 {
        return false;
    }

    if row as usize >= grid.len() || col as usize >= grid[0].len() {
        return false;
    }

    true
}

fn is_reachable(
    grid: &Vec<Vec<usize>>,
    curr_row: i32,
    curr_col: i32,
    next_row: i32,
    next_col: i32,
) -> bool {
    if !is_in_bounds(grid, next_row as i32, next_col as i32) {
        return false;
    }

    if grid[next_row as usize][next_col as usize] != grid[curr_row as usize][curr_col as usize] + 1
    {
        return false;
    }

    true
}

fn dfs(
    grid: &Vec<Vec<usize>>,
    row: i32,
    col: i32,
    rows: usize,
    cols: usize,
    trail: &mut Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    if trail.contains(&(row as usize, col as usize)) {
        return Vec::new();
    }

    trail.push((row as usize, col as usize));

    let mut trails: Vec<Vec<(usize, usize)>> = Vec::new();

    if grid[row as usize][col as usize] == 9 {
        trails.push(trail.clone());
    } else {
        let directions = vec![
            (1, 0),  // Down
            (0, 1),  // Right
            (-1, 0), // Up
            (0, -1), // Left
        ];

        for (dr, dc) in directions {
            let next_row = row + dr;
            let next_col = col + dc;

            if is_reachable(grid, row, col, next_row, next_col) {
                trails.extend(dfs(grid, next_row, next_col, rows, cols, trail));
            }
        }
    }

    // Backtrack
    trail.pop();
    trails
}

fn solution(input: &str) -> String {
    let mut start_points: Vec<(usize, usize)> = Vec::new();
    let grid = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    let digit = c.to_digit(10).unwrap() as usize;
                    if digit == 0 {
                        start_points.push((row, col));
                    }
                    digit
                })
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut trail: Vec<(usize, usize)> = Vec::new();
    let mut unique_trails: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    start_points.iter().for_each(|(row, col)| {
        let trails = dfs(
            &grid,
            *row as i32,
            *col as i32,
            grid.len(),
            grid[0].len(),
            &mut trail,
        );

        for trail in trails {
            if let (Some(&start), Some(&end)) = (trail.first(), trail.last()) {
                unique_trails.insert((start, end));
            }
        }
    });

    unique_trails.len().to_string()
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
