fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> String {
    let sequences: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    dbg!(sequences.clone());

    sequences
        .iter()
        .map(|sequence| {
            let mut diff_seqs: Vec<Vec<i32>> = Vec::new();
            diff_seqs.push(sequence.clone());
            loop {
                let last_diff_seq = diff_seqs.last().unwrap();
                let mut diff_seq: Vec<i32> = Vec::new();
                for (i, num) in last_diff_seq.iter().enumerate().skip(1) {
                    diff_seq.push(num - last_diff_seq[i - 1]);
                }
                let stop = diff_seq.iter().all(|&num| num == 0);

                println!("{:?}", diff_seq);

                diff_seqs.push(diff_seq);

                if stop {
                    break;
                }
            }

            diff_seqs
                .iter()
                .rev()
                .skip(1)
                .fold(0, |mut acc: i32, diff_seq: &Vec<i32>| {
                    acc += diff_seq.last().unwrap();
                    acc
                })
        })
        .sum::<i32>()
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
