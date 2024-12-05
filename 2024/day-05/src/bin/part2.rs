use std::collections::HashMap;

fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> String {
    let parts = input.split_once("\n\n").unwrap();

    let rules: Vec<(i32, i32)> = parts
        .0
        .lines()
        .map(|line| {
            line.split_once("|")
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect();
    let updates: Vec<Vec<i32>> = parts
        .1
        .lines()
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    updates
        .iter()
        .filter(|update| {
            let mut is_correct = true;
            for (a, b) in &rules {
                if !update.contains(a) || !update.contains(b) {
                    continue;
                }
                if update.iter().position(|&x| x == *a).unwrap()
                    > update.iter().position(|&x| x == *b).unwrap()
                {
                    is_correct = false;
                    break;
                }
            }
            !is_correct
        })
        .into_iter()
        .map(|update| {
            let applied_rules = rules
                .iter()
                .filter(|(a, b)| update.contains(a) && update.contains(b))
                .collect::<Vec<_>>();

            let mut vertices_in_degree: HashMap<i32, i32> = applied_rules
                .iter()
                .flat_map(|(a, b)| vec![*a, *b])
                .map(|v| (v, 0))
                .collect();
            for (_, b) in &applied_rules {
                *vertices_in_degree.entry(*b).or_insert(-1) += 1;
            }

            let mut queue: Vec<i32> = Vec::new();
            let mut order: Vec<i32> = Vec::new();

            loop {
                let zero_in_degree_vertices: Vec<i32> = vertices_in_degree
                    .iter()
                    .filter(|(_, &in_degree)| in_degree == 0)
                    .map(|(&v, _)| v)
                    .collect();

                if !zero_in_degree_vertices.is_empty() {
                    for v in &zero_in_degree_vertices {
                        vertices_in_degree.remove(v);
                        queue.push(*v);
                    }
                }

                if !queue.is_empty() {
                    let v = queue.pop().unwrap();
                    order.push(v);

                    let neighbors = applied_rules
                        .iter()
                        .filter(|(a, _)| *a == v)
                        .map(|(_, b)| b)
                        .collect::<Vec<_>>();

                    for neighbor in neighbors {
                        if let Some(in_degree) = vertices_in_degree.get_mut(neighbor) {
                            *in_degree -= 1;
                        }
                    }
                } else {
                    break;
                }
            }

            let mut order_iter = order.iter();

            update
                .iter()
                .map(|&v| {
                    if order.contains(&v) {
                        *order_iter.next().unwrap()
                    } else {
                        v
                    }
                })
                .collect::<Vec<_>>()
        })
        .map(|update| update[update.len() / 2])
        .sum::<i32>()
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
