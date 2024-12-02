fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn get_n_winning_ways(&self) -> u64 {
        (1..=self.time)
            .filter(|&x| (self.time - x) * x > self.distance)
            .count() as u64
    }
}

fn solution(input: &str) -> u64 {
    let mut lines = input.lines();

    let parse_line = |line: Option<&str>| -> u64 {
        line.and_then(|line| line.split(":").last())
            .and_then(|s| s.replace(" ", "").parse::<u64>().ok())
            .unwrap_or_default()
    };

    let time = parse_line(lines.next());
    let distance = parse_line(lines.next());

    let race = Race { time, distance };

    race.get_n_winning_ways()
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
