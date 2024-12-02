fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn solution(input: &str) -> String {
    let mut lines = input.lines();

    let mut start_nodes: Vec<&str> = Vec::new();
    let mut two_next_nodes: Vec<(&str, &str)> = Vec::new();

    let mut current_node_idx = 0;
    let mut dest_node_idx = 0;

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
        // Get next nodes BBB CCC from "(BBB, CCC)"
        let next_nodes: (&str, &str) = next_part
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split_once(", ")
            .unwrap();

        if start_node == "AAA" {
            current_node_idx = start_nodes.len();
        }

        if start_node == "ZZZ" {
            dest_node_idx = start_nodes.len();
        }

        start_nodes.push(start_node);
        two_next_nodes.push(next_nodes);
    });

    let mut steps = 0;

    // While we have not reached the end node
    while current_node_idx != dest_node_idx {
        for direction in instruction.iter() {
            dbg!(&current_node_idx);

            let next_node = match direction {
                Direction::Left => two_next_nodes[current_node_idx].0,
                Direction::Right => two_next_nodes[current_node_idx].1,
            };

            let next_node_idx = start_nodes
                .iter()
                .position(|&node| node == next_node)
                .unwrap();

            current_node_idx = next_node_idx;

            steps += 1;

            if current_node_idx == dest_node_idx {
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
        let input = include_str!("./tests/input1.txt");
        let output = solution(input).to_string();
        let expected_output = include_str!("./tests/output1.txt");

        assert_eq!(output, expected_output);
    }
}
