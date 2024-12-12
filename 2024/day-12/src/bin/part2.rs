use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./inputs/input2.txt");
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

fn is_border_cell(grid: &Vec<Vec<char>>, x: usize, y: usize, island_type: char) -> bool {
    let neighbors = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for (dx, dy) in neighbors {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx < 0 || ny < 0 || nx >= grid.len() as i32 || ny >= grid[0].len() as i32 {
            return true;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if grid[nx][ny] != island_type {
            return true;
        }
    }

    false
}

fn is_connected(a: &(usize, usize), b: &(usize, usize)) -> bool {
    let (x1, y1) = a;
    let (x2, y2) = b;

    if x1 == x2 {
        return y1 + 1 == *y2 || *y1 == y2 + 1;
    }

    if y1 == y2 {
        return x1 + 1 == *x2 || *x1 == x2 + 1;
    }

    false
}

fn count_disjoint_x_seq(seq: &Vec<(usize, usize)>) -> usize {
    if seq.len() < 1 {
        return 0;
    } else if seq.len() == 1 {
        return 1;
    }

    let mut disjoint_sequences = 1;

    let mut seq = seq.clone();
    seq.sort_by(|a, b| a.0.cmp(&b.0));

    for i in 1..seq.len() {
        if is_connected(&seq[i], &seq[i - 1]) {
            continue;
        } else {
            disjoint_sequences += 1;
        }
    }

    disjoint_sequences
}

fn count_disjoint_y_seq(seq: &Vec<(usize, usize)>) -> usize {
    if seq.len() < 1 {
        return 0;
    } else if seq.len() == 1 {
        return 1;
    }

    let mut disjoint_sequences = 1;

    let mut seq = seq.clone();
    seq.sort_by(|a, b| a.1.cmp(&b.1));

    for i in 1..seq.len() {
        if is_connected(&seq[i], &seq[i - 1]) {
            continue;
        } else {
            disjoint_sequences += 1;
        }
    }

    disjoint_sequences
}

fn get_distinct_edges(
    grid: &Vec<Vec<char>>,
    island: &Vec<(usize, usize)>,
    island_type: char,
) -> usize {
    let mut boundary_cells: Vec<(usize, usize)> = Vec::with_capacity(island.len());
    for (x, y) in island {
        if is_border_cell(grid, x.clone(), y.clone(), island_type) {
            boundary_cells.push((x.clone(), y.clone()));
        }
    }

    if boundary_cells.len() <= 2 {
        return 4;
    }

    let max_x = boundary_cells.iter().max_by_key(|(x, _)| x).unwrap().0;
    let min_x = boundary_cells.iter().min_by_key(|(x, _)| x).unwrap().0;

    let max_y = boundary_cells.iter().max_by_key(|(_, y)| y).unwrap().1;
    let min_y = boundary_cells.iter().min_by_key(|(_, y)| y).unwrap().1;

    let mut distinct_edges = 0;

    let mut prev_y_values: HashSet<usize> = HashSet::new();
    (min_x..=max_x).for_each(|x| {
        let current_y_values: HashSet<usize> = island
            .iter()
            .filter(|(x_val, _)| x == *x_val)
            .map(|(_, y_val)| *y_val)
            .collect();

        let y_values: Vec<(usize, usize)> = current_y_values
            .symmetric_difference(&prev_y_values)
            .cloned()
            .map(|val| {
                if current_y_values.contains(&val) {
                    (x, val)
                } else {
                    (x - 1, val)
                }
            })
            .collect();

        let disjoint_sequences = count_disjoint_y_seq(&y_values);
        distinct_edges += disjoint_sequences;

        if x == max_x {
            distinct_edges +=
                count_disjoint_y_seq(&current_y_values.iter().map(|val| (x, *val)).collect());
        }

        prev_y_values = current_y_values;
    });

    let mut prev_x_values: HashSet<usize> = HashSet::new();
    (min_y..=max_y).for_each(|y| {
        let current_x_values: HashSet<usize> = island
            .iter()
            .filter(|(_, y_val)| y == *y_val)
            .map(|(x_val, _)| *x_val)
            .collect();

        // Perform symmetric difference between current and previous x_values
        let x_values: Vec<(usize, usize)> = current_x_values
            .symmetric_difference(&prev_x_values)
            .cloned()
            .map(|val| {
                if current_x_values.contains(&val) {
                    (val, y)
                } else {
                    (val, y - 1)
                }
            })
            .collect();

        // Count disjoint sequences based on the new x_values
        let disjoint_sequences = count_disjoint_x_seq(&x_values);
        distinct_edges += disjoint_sequences;

        if y == max_y {
            distinct_edges +=
                count_disjoint_x_seq(&current_x_values.iter().map(|val| (*val, y)).collect());
        }

        // Update prev_x_values for the next iteration
        prev_x_values = current_x_values;
    });

    distinct_edges
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
            let perimeter = get_distinct_edges(&grid, island, grid[island[0].0][island[0].1]);
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
        let input = include_str!("./tests/input2.txt");
        let output = solution(input).to_string();
        let expected_output = include_str!("./tests/output2.txt");

        assert_eq!(output, expected_output);
    }
}
