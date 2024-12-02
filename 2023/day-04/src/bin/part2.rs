fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug)]
struct Card {
    idx: u32,
    amount: u32,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

fn add_to_cards(cards: &mut Vec<Card>, idx: u32, amount: u32) {
    match cards.iter().find(|card| card.idx == idx) {
        Some(card) => {
            let card_idx = card.idx as usize;
            cards[card_idx].amount += amount;
        }
        None => {
            cards.push(Card {
                idx,
                amount: amount,
            });
        }
    }
}

fn solution(input: &str) -> u32 {
    let mut cards: Vec<Card> = Vec::new();

    let max_card_idx = input.lines().count();

    input.lines().enumerate().for_each(|(idx, line)| {
        add_to_cards(&mut cards, idx as u32, 1);

        let number_parts = line
            .split(":")
            .last()
            .expect("No number part")
            .split("|")
            .collect::<Vec<_>>();

        let winning_numbers = number_parts[0]
            .split_whitespace()
            .map(|number| number.parse::<u32>().expect("Not a number"))
            .collect::<Vec<_>>();

        let n_matches = number_parts[1]
            .split_whitespace()
            .map(|number_str| {
                let num = number_str.parse::<u32>().expect("Not a number");

                winning_numbers.contains(&num) as u32
            })
            .sum::<u32>();

        (idx + 1..idx + 1 + n_matches as usize).for_each(|added_idx| {
            if added_idx <= max_card_idx {
                let amount = cards[idx].amount;
                add_to_cards(&mut cards, added_idx as u32, amount);
            }
        });
    });

    cards.iter().map(|card| card.amount).sum::<u32>()
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
