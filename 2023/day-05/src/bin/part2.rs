fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug)]
struct SeedPair {
    start: u64,
    range_len: u64,
}

struct MapTriple {
    dest_start: u64,
    source_start: u64,
    range_len: u64,
}

struct Map {
    triples: Vec<MapTriple>,
}

impl Map {
    fn extend_map(&mut self, dest_start: u64, source_start: u64, range_len: u64) -> &mut Self {
        self.triples.push(MapTriple {
            dest_start,
            source_start,
            range_len,
        });
        self
    }

    fn get_value(&self, key: &u64) -> u64 {
        let mut value = *key;
        for triple in &self.triples {
            if triple.source_start <= value && value < triple.source_start + triple.range_len {
                value = triple.dest_start + (value - triple.source_start);
                break;
            }
        }
        value
    }

    fn get_key(&self, value: &u64) -> u64 {
        let mut key = *value;
        for triple in self.triples.iter().rev() {
            if triple.dest_start <= key && key < triple.dest_start + triple.range_len {
                key = triple.source_start + (key - triple.dest_start);
                break;
            }
        }
        key
    }
}

struct LocationMap {
    maps: Vec<Map>,
}

impl LocationMap {
    fn to_location(&self, seed: u64) -> u64 {
        let mut key = seed;
        for map in &self.maps {
            key = map.get_value(&key);
        }
        key
    }

    fn to_seed(&self, location: u64) -> u64 {
        let mut key = location;
        for map in self.maps.iter().rev() {
            key = map.get_key(&key);
        }
        key
    }
}

fn in_range(seed: u64, seed_pair: &SeedPair) -> bool {
    seed_pair.start <= seed && seed < seed_pair.start + seed_pair.range_len
}

fn solution(input: &str) -> u64 {
    let seed_pairs: Vec<SeedPair> = input
        .lines()
        .next()
        .expect("No input")
        .split(":")
        .last()
        .expect("No seeds")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|pair| SeedPair {
            start: pair[0].parse().expect("Invalid seed"),
            range_len: pair[1].parse().expect("Invalid seed"),
        })
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

        part.trim().lines().skip(1).for_each(|line| {
            let nums = line
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid number"))
                .collect::<Vec<u64>>();

            let dest_start = nums[0];
            let source_start = nums[1];
            let range_len = nums[2];

            map.extend_map(dest_start, source_start, range_len);
        });

        maps.push(map);
    }

    let location_map = LocationMap { maps };

    (0..)
        .into_iter()
        .find(|&location| {
            let seed = location_map.to_seed(location);
            seed_pairs.iter().any(|seed_pair| in_range(seed, seed_pair))
        })
        .unwrap()
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
