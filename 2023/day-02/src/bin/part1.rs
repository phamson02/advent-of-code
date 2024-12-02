use std::result::Result;

fn main() {
    let input = include_str!("./inputs/input1.txt");
    let output = solution(input);
    dbg!(output);
}

struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn remove(&mut self, red: u32, green: u32, blue: u32) -> Result<(), &'static str> {
        if self.red < red || self.green < green || self.blue < blue {
            Err("Not enough items in the bag")
        } else {
            self.red -= red;
            self.green -= green;
            self.blue -= blue;
            Ok(())
        }
    }
}

fn solution(input: &str) -> u32 {
    let output: u32 = input
        .lines()
        .map(|line| {
            // First split the line by :, the first part is game name, the second part is the number of items
            let mut parts = line.split(':');

            // Get the number which is the game name in the first part
            let game_name = parts.next().expect("No game name found");

            // Get the game index from the game name, which can be number with multiple digits
            let mut game_index = 0;
            for c in game_name.chars() {
                if let Some(digit) = c.to_digit(10) {
                    game_index = game_index * 10 + digit;
                }
            }

            let parts = parts.next().expect("No rounds found").split(';');

            for part in parts {
                for item_part in part.split(',') {
                    let mut bag = Bag {
                        red: 12,
                        green: 13,
                        blue: 14,
                    };
                    // item part is like "1 red", "2 green", "3 blue"
                    let mut item_part = item_part.split_whitespace();
                    let item_count = item_part
                        .next()
                        .expect("No item count found")
                        .parse::<u32>()
                        .expect("Item count is not a number");
                    let item_name = item_part.next().expect("No item name found");

                    match item_name {
                        "red" => bag.remove(item_count, 0, 0),
                        "green" => bag.remove(0, item_count, 0),
                        "blue" => bag.remove(0, 0, item_count),
                        _ => panic!("Unknown item name"),
                    }
                    .unwrap_or_else(|_| {
                        game_index = 0;
                    });
                }

                if game_index == 0 {
                    break;
                }
            }
            game_index
        })
        .sum();

    output
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
