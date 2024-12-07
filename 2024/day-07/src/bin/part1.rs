fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

struct OperatorGenerator {
    n_ops: i32,
    n_ops_type: i32,
    total_ops_permutations: i32,
    current_op: i32,
}

impl OperatorGenerator {
    fn new(n_ops: i32, n_ops_type: i32) -> Self {
        let total_ops_permutations = n_ops_type.pow(n_ops as u32);
        Self {
            n_ops,
            n_ops_type,
            total_ops_permutations,
            current_op: 0,
        }
    }
}

impl Iterator for OperatorGenerator {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_op == self.total_ops_permutations {
            return None;
        }

        let mut op = self.current_op;
        let mut ops = Vec::with_capacity(self.n_ops as usize);
        for _ in 0..self.n_ops {
            ops.push(op % self.n_ops_type);
            op /= self.n_ops_type;
        }
        self.current_op += 1;
        Some(ops)
    }
}

fn check(target: i64, numbers: Vec<i64>) -> i64 {
    fn evaluate(nums: &[i64], ops: &[i32], target: i64) -> i64 {
        let mut result = nums[0];
        for i in 0..ops.len() {
            if result > target {
                return 0;
            }

            if ops[i] == 0 {
                result += nums[i + 1];
            } else {
                result *= nums[i + 1];
            }
        }
        result
    }

    let n_ops = numbers.len() - 1;

    for ops in OperatorGenerator::new(n_ops as i32, 2) {
        if evaluate(&numbers, &ops, target) == target {
            return target;
        }
    }
    0
}

fn solution(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let target = parts.next().unwrap().parse().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|n| n.parse().ok())
                .collect::<Option<Vec<i64>>>()
                .unwrap();

            check(target, numbers)
        })
        .sum::<i64>()
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
