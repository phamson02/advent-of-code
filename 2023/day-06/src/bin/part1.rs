fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn get_n_winning_ways(&self) -> u32 {
        (1..=self.time)
            .filter(|&x| (self.time - x) * x > self.distance)
            .count() as u32
    }
}

fn solution(input: &str) -> u32 {
    let lines: Vec<_> = input
        .lines()
        .map(|line| {
            line.split(":")
                .last()
                .unwrap()
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let (times, distances) = match lines.as_slice() {
        [times, distances, ..] => (times.clone(), distances.clone()),
        _ => panic!("Expected at least two lines"),
    };

    let races = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect::<Vec<Race>>();

    races
        .iter()
        .map(|race| race.get_n_winning_ways())
        .product::<u32>()
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
