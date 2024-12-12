use std::{collections::HashMap, vec};

fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn dfs(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    islands: &mut HashMap<char, Vec<Vec<(usize, usize)>>>,
    visited: &mut Vec<Vec<bool>>,
    current_val: char,
) {
    let neighbors = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    visited[x][y] = true;
    islands
        .entry(current_val)
        .and_modify(|island| island.last_mut().unwrap().push((x, y)));

    for (dx, dy) in neighbors {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx < 0 || ny < 0 || nx >= grid.len() as i32 || ny >= grid[0].len() as i32 {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if visited[nx][ny] {
            continue;
        }

        if current_val != grid[nx][ny] {
            continue;
        }
        dfs(grid, nx, ny, islands, visited, current_val);
    }
}

fn get_perimeter(grid: &Vec<Vec<char>>, island: &Vec<(usize, usize)>, island_type: char) -> usize {
    let mut perimeter = 4 * island.len();

    for (x, y) in island {
        let neighbors = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (dx, dy) in neighbors {
            let nx = *x as i32 + dx;
            let ny = *y as i32 + dy;

            if nx < 0 || ny < 0 || nx >= grid.len() as i32 || ny >= grid[0].len() as i32 {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if grid[nx][ny] == island_type {
                perimeter -= 1;
            }
        }
    }

    perimeter
}

fn solution(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut islands: HashMap<char, Vec<Vec<(usize, usize)>>> = HashMap::default();

    let mut current_value = ' ';
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if visited[x][y] {
                continue;
            }

            if grid[x][y] != current_value
                || !islands
                    .entry(current_value)
                    .or_default()
                    .last()
                    .unwrap_or(&vec![])
                    .contains(&(x, y))
            {
                current_value = grid[x][y].clone();
                islands.entry(current_value).or_default().push(vec![]);
            }

            dfs(&grid, x, y, &mut islands, &mut visited, current_value);
        }
    }

    islands
        .values()
        .flatten()
        .map(|island| {
            let area = island.len();
            let perimeter = get_perimeter(&grid, island, grid[island[0].0][island[0].1]);
            dbg!(area, perimeter);

            area * perimeter
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
