fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn to_layout_str(disk_map: &Vec<usize>) -> Vec<char> {
    let mut layout: Vec<char> = Vec::new();
    let mut curr_idx = '.' as usize + 1;
    disk_map.iter().enumerate().for_each(|(i, &n)| {
        if i % 2 == 0 {
            // push 'curr_idx' n times
            for _ in 0..n {
                layout.push(std::char::from_u32(curr_idx as u32).unwrap());
            }
        } else {
            // push '.' n times
            for _ in 0..n {
                layout.push('.');
            }
            curr_idx += 1;
        }
    });
    layout
}

fn move_layout_str(layout: &Vec<char>) -> Vec<char> {
    let mut new_layout: Vec<char> = Vec::new();
    let mut left_idx = 0;
    let mut right_idx = layout.len() - 1;

    while left_idx <= right_idx {
        if layout[left_idx] != '.' {
            new_layout.push(layout[left_idx]);
        } else {
            while right_idx > left_idx && layout[right_idx] == '.' {
                right_idx -= 1;
            }
            if layout[right_idx] != '.' {
                new_layout.push(layout[right_idx]);
                right_idx -= 1;
            }
        }
        left_idx += 1;
    }

    new_layout
}

fn solution(input: &str) -> String {
    let disk_map: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let layout = to_layout_str(&disk_map);

    let new_layout = move_layout_str(&layout);

    new_layout
        .iter()
        .map(|&c| c as u32 - ('.' as u32 + 1))
        .collect::<Vec<u32>>()
        .iter()
        .enumerate()
        .fold(0, |acc: u64, (i, &n)| acc + (i as u32 * n) as u64)
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
