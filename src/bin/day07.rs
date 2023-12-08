use std::cmp::Ordering;

use ahash::AHashMap;

type Card = (u32, char);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    HighCard,
    OnePair,
    TwoPairs,
    Three,
    FullHouse,
    Four,
    Five,
}

fn parse_cards(input: &str) -> [Card; 5] {
    let mut cards = [(0, '_'); 5];

    for (card, ch) in cards.iter_mut().zip(input.chars()) {
        *card = match ch {
            ch if ch.is_ascii_digit() => (ch.to_digit(10).unwrap(), ch),
            'T' => (10, 'T'),
            'J' => (11, 'J'),
            'Q' => (12, 'Q'),
            'K' => (13, 'K'),
            'A' => (14, 'A'),
            _ => unreachable!(),
        };
    }

    cards
}

fn parse(input: &str) -> Vec<([Card; 5], usize)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_ascii_whitespace();
            if let (Some(first), Some(second)) = (split.next(), split.next()) {
                (parse_cards(first), second.parse().unwrap())
            } else {
                panic!("parse error!")
            }
        })
        .collect()
}

fn card_type(cards: &[Card]) -> CardType {
    let mut counts = AHashMap::new();

    for card in cards {
        *counts.entry(card).or_insert(0) += 1;
    }

    let same_cards = if let Some(same) = counts.iter().map(|e| e.1).max() {
        *same
    } else {
        return CardType::HighCard;
    };

    if same_cards == 5 {
        CardType::Five
    } else if same_cards == 4 {
        CardType::Four
    } else if same_cards == 3 {
        if counts.iter().filter(|e| *e.1 == 2).count() == 1 {
            CardType::FullHouse
        } else {
            CardType::Three
        }
    } else if same_cards == 2 {
        if counts.iter().filter(|e| *e.1 == 2).count() == 2 {
            CardType::TwoPairs
        } else {
            CardType::OnePair
        }
    } else {
        CardType::HighCard
    }
}

fn total_winnings(hands: Vec<([(u32, char); 5], usize)>) -> usize {
    hands
        .iter()
        .enumerate()
        .map(|(rank, h)| (rank + 1) * h.1)
        .sum()
}

fn part1(mut hands: Vec<([Card; 5], usize)>) -> usize {
    hands.sort_by(|(left, _), (right, _)| {
        let type_left = card_type(left);
        let type_right = card_type(right);

        if type_left == type_right {
            for (left, right) in left.iter().zip(right.iter()) {
                let ord = left.cmp(right);
                if ord != Ordering::Equal {
                    return ord;
                }
            }

            Ordering::Equal
        } else {
            type_left.cmp(&type_right)
        }
    });

    total_winnings(hands)
}

fn part2(mut hands: Vec<([Card; 5], usize)>) -> usize {
    hands.sort_by(|(left, _), (right, _)| {
        let no_joker_left: Vec<_> =
            left.iter().filter(|c| c.1 != 'J').copied().collect();
        let no_joker_right: Vec<_> =
            right.iter().filter(|c| c.1 != 'J').copied().collect();
        let mut type_left = card_type(no_joker_left.as_slice());
        let mut type_right = card_type(no_joker_right.as_slice());

        type_left = use_jokers(type_left, left.len() - no_joker_left.len());
        type_right = use_jokers(type_right, right.len() - no_joker_right.len());

        if type_left == type_right {
            for (left, right) in left.iter().zip(right.iter()) {
                let ord = left.cmp(right);
                if ord != Ordering::Equal {
                    return if left.1 == 'J' {
                        Ordering::Less
                    } else if right.1 == 'J' {
                        Ordering::Greater
                    } else {
                        ord
                    };
                }
            }

            Ordering::Equal
        } else {
            type_left.cmp(&type_right)
        }
    });

    total_winnings(hands)
}

fn use_jokers(tl: CardType, num_jokers: usize) -> CardType {
    match (tl, num_jokers) {
        (CardType::Five, _) => CardType::Five,
        (CardType::Four, 1) => CardType::Five,
        (CardType::FullHouse, _) => CardType::FullHouse,
        (CardType::Three, 1) => CardType::Four,
        (CardType::Three, 2) => CardType::Five,
        (CardType::TwoPairs, 1) => CardType::FullHouse,
        (CardType::OnePair, 3) => CardType::Five,
        (CardType::OnePair, 2) => CardType::Four,
        (CardType::OnePair, 1) => CardType::Three,
        (CardType::HighCard, 1) => CardType::OnePair,
        (CardType::HighCard, 2) => CardType::Three,
        (CardType::HighCard, 3) => CardType::Four,
        (CardType::HighCard, 4) => CardType::Five,
        (CardType::HighCard, 5) => CardType::Five,
        (t, 0) => t,
        _ => {
            unreachable!()
        }
    }
}

fn main() {
    let input = include_str!("../../input/input07.txt");
    let input = parse(input);
    println!("part1 = {}", part1(input.clone()));
    println!("part2 = {}", part2(input));
}

#[test]
fn test_day07() {
    let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let input = parse(input);

    assert_eq!(part1(input.clone()), 6440);
    assert_eq!(part2(input), 5905);
}
