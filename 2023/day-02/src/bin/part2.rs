fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn extend(&mut self, red: u32, green: u32, blue: u32) {
        if self.red < red {
            self.red = red;
        }
        if self.green < green {
            self.green = green;
        }
        if self.blue < blue {
            self.blue = blue;
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

            let mut bag = Bag {
                red: 0,
                green: 0,
                blue: 0,
            };

            for part in parts {
                for item_part in part.split(',') {
                    // item part is like "1 red", "2 green", "3 blue"
                    let mut item_part = item_part.split_whitespace();
                    let item_count = item_part
                        .next()
                        .expect("No item count found")
                        .parse::<u32>()
                        .expect("Item count is not a number");
                    let item_name = item_part.next().expect("No item name found");

                    match item_name {
                        "red" => bag.extend(item_count, 0, 0),
                        "green" => bag.extend(0, item_count, 0),
                        "blue" => bag.extend(0, 0, item_count),
                        _ => panic!("Unknown item name"),
                    }
                }
            }
            bag.red * bag.green * bag.blue
        })
        .sum();

    output
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
