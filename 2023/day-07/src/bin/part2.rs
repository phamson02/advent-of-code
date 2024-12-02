use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./inputs/input2.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn new(cards: Vec<char>, bid: u32) -> Self {
        let cards = cards
            .into_iter()
            .map(|card| match card {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::T,
                'J' => Card::J,
                'Q' => Card::Q,
                'K' => Card::K,
                'A' => Card::A,
                _ => unreachable!("Invalid card: {}", card),
            })
            .collect();

        Self { cards, bid }
    }

    fn hand_type(&self) -> HandType {
        let mut counter = HashMap::new();
        let mut j_count = 0;

        for card in &self.cards {
            if card == &Card::J {
                j_count += 1;
                continue;
            }

            let count = counter.entry(card).or_insert(0);
            *count += 1;
        }

        let mut counts = counter.values().cloned().collect::<Vec<_>>();

        counts.sort();

        if let Some(last) = counts.last_mut() {
            *last += j_count;
        } else {
            counts.push(j_count);
        }

        match counts.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 2, 2] => HandType::TwoPairs,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 4] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => unreachable!("Invalid hand: {:?}", self),
        }
    }

    fn has_stronger_card(&self, other: &Self) -> bool {
        let self_cards = self.cards.iter();
        let other_cards = other.cards.iter();

        for (self_card, other_card) in self_cards.zip(other_cards) {
            if self_card != other_card {
                return self_card > other_card;
            }
        }

        false
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        !self.has_stronger_card(other)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();

        if self_type != other_type {
            return self_type.cmp(&other_type);
        }

        if self.has_stronger_card(other) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

fn solution(input: &str) -> String {
    let mut hands = Vec::new();

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        let cards = parts.next().unwrap().chars().collect::<Vec<_>>();
        let bid = parts.next().unwrap();

        let hand = Hand::new(cards, bid.parse().unwrap());
        hands.push(hand);
    });

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum::<u32>()
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
