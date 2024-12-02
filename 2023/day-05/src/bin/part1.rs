fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

struct MapTriple {
    dest_start: u32,
    source_start: u32,
    range_len: u32,
}

struct Map {
    triples: Vec<MapTriple>,
}

impl Map {
    fn extend_map(&mut self, dest_start: u32, source_start: u32, range_len: u32) -> &mut Self {
        self.triples.push(MapTriple {
            dest_start,
            source_start,
            range_len,
        });
        self
    }

    fn get_value(&self, key: &u32) -> u32 {
        let mut value = *key;
        for triple in &self.triples {
            if triple.source_start <= value && value < triple.source_start + triple.range_len {
                value = triple.dest_start + (value - triple.source_start);
                break;
            }
        }
        value
    }
}

struct LocationMap {
    maps: Vec<Map>,
}

impl LocationMap {
    fn to_location(&self, seed: u32) -> u32 {
        let mut key = seed;
        for map in &self.maps {
            key = map.get_value(&key);
        }
        key
    }
}

fn solution(input: &str) -> u32 {
    let seeds: Vec<u32> = input
        .lines()
        .next()
        .expect("No input")
        .split(":")
        .last()
        .expect("No seeds")
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid seed"))
        .collect();

    let mut maps = Vec::new();

    // Process input by parts splitted by empty lines, skip first line
    for part in input
        .lines()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("\n")
        .split("\n\n")
    {
        let mut map = Map {
            triples: Vec::new(),
        };

        dbg!(part.trim());

        part.trim().lines().skip(1).for_each(|line| {
            let nums = line
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid number"))
                .collect::<Vec<u32>>();

            let dest_start = nums[0];
            let source_start = nums[1];
            let range_len = nums[2];

            map.extend_map(dest_start, source_start, range_len);
        });

        maps.push(map);
    }

    let location_map = LocationMap { maps };

    seeds
        .iter()
        .map(|seed| location_map.to_location(*seed))
        .min()
        .expect("No seeds")
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
