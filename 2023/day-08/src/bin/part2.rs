// use crate num::integer::lcm;
use num::integer::lcm;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn is_completed(current_node_idxs: &Vec<u32>, start_nodes: &Vec<&str>) -> bool {
    current_node_idxs
        .iter()
        .all(|&idx| start_nodes[idx as usize].chars().last().unwrap() == 'Z')
}

fn solution(input: &str) -> String {
    let mut lines = input.lines();

    let mut start_nodes: Vec<&str> = Vec::new();
    let mut two_next_nodes: Vec<(&str, &str)> = Vec::new();

    let mut current_node_idxs: Vec<u32> = Vec::new();
    let mut dest_node_idxs: Vec<u32> = Vec::new();

    let instruction: Vec<Direction> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect();

    lines.skip(1).for_each(|line| {
        let (start_node, next_part) = line.split_once(" = ").unwrap();

        let next_nodes: (&str, &str) = next_part
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split_once(", ")
            .unwrap();

        if start_node.chars().last().unwrap() == 'A' {
            current_node_idxs.push(start_nodes.len() as u32)
        }
        if start_node.chars().last().unwrap() == 'Z' {
            dest_node_idxs.push(start_nodes.len() as u32)
        }

        start_nodes.push(start_node);
        two_next_nodes.push(next_nodes);
    });

    dbg!(&current_node_idxs);
    dbg!(&dest_node_idxs);

    let mut dest_steps_list = HashMap::new();
    let mut steps = 0;

    while !is_completed(&current_node_idxs, &start_nodes) {
        for direction in instruction.iter() {
            let next_nodes: Vec<&str> = match direction {
                Direction::Left => current_node_idxs
                    .iter()
                    .map(|&idx| two_next_nodes[idx as usize].0)
                    .collect(),
                Direction::Right => current_node_idxs
                    .iter()
                    .map(|&idx| two_next_nodes[idx as usize].1)
                    .collect(),
            };

            let next_node_idxs = next_nodes
                .iter()
                .map(|&node| start_nodes.iter().position(|&x| x == node).unwrap() as u32)
                .collect();

            current_node_idxs = next_node_idxs;

            steps += 1;

            // Check if any of the current nodes are the destination nodes
            for &current_node_idx in current_node_idxs.iter() {
                if dest_node_idxs.contains(&current_node_idx)
                    && !dest_steps_list.contains_key(&current_node_idx)
                {
                    dest_steps_list.insert(current_node_idx, steps);
                    println!(
                        "Found destination node {} at step {}",
                        start_nodes[current_node_idx as usize], steps
                    );
                }

                if dest_steps_list.len() == dest_node_idxs.len() {
                    // steps = LCM of all values in dest_steps_list
                    steps = dest_steps_list
                        .values()
                        .fold(1, |acc, &x| lcm(acc, x) as u64);
                    return steps.to_string();
                }
            }

            if is_completed(&current_node_idxs, &start_nodes) {
                break;
            }
        }
    }

    steps.to_string()
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
