fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

fn to_layout_str(disk_map: &Vec<usize>) -> Vec<char> {
    let mut layout: Vec<char> = Vec::new();
    let mut curr_idx = '.' as u32 + 1;
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

#[derive(Debug)]
struct File {
    id: u32,
    size: usize,
    pos: usize,
}

fn move_layout_str(layout: &Vec<char>) -> Vec<char> {
    let mut curr_idx = 0;
    let mut files: Vec<File> = Vec::new();
    layout
        .iter()
        .enumerate()
        .filter(|(_, &c)| c != '.')
        .for_each(|(i, &c)| {
            let id = c as u32;
            if curr_idx != id {
                let size = 1;
                let pos = i;
                files.push(File { id, size, pos });
                curr_idx = id;
            } else {
                files.last_mut().unwrap().size += 1;
            }
        });

    let mut new_layout = layout.clone();

    files.iter().rev().for_each(|f| {
        let mut space_size = 0;
        let mut space_pos = 0;
        for (i, &c) in new_layout.iter().enumerate() {
            if i >= f.pos {
                break;
            }

            if c == '.' {
                if space_size == 0 {
                    space_pos = i;
                }
                space_size += 1;
            } else if space_size > 0 {
                if space_size >= f.size {
                    break;
                }
                space_size = 0;
            } else {
                space_size = 0;
            }
        }
        if space_size >= f.size {
            (0..f.size).for_each(|i| {
                new_layout[space_pos + i] = std::char::from_u32(f.id).unwrap();
                new_layout[f.pos + i] = '.';
            });
        }
    });

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
        .map(|&c| {
            if c == '.' {
                0
            } else {
                (c as u64).checked_sub('.' as u64 + 1).unwrap()
            }
        })
        .collect::<Vec<u64>>()
        .iter()
        .enumerate()
        .fold(0, |acc: u64, (i, &n)| acc + (i as u64 * n) as u64)
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
