use ahash::AHashSet;

fn parse(input: &str) -> Vec<(AHashSet<usize>, AHashSet<usize>)> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (AHashSet<usize>, AHashSet<usize>) {
    let mut numbers = line.split(|c| c == ':' || c == '|').skip(1);

    let winning = numbers.next().unwrap();
    let mine = numbers.next().unwrap();

    (
        winning
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect(),
        mine.split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect(),
    )
}

fn part1(cards: &[(AHashSet<usize>, AHashSet<usize>)]) -> usize {
    let mut sum = 0;

    for (winning, mine) in cards {
        let count = winning.intersection(mine).count();
        if count > 0 {
            sum += 1 << (count - 1)
        }
    }

    sum
}

fn part2(cards: &[(AHashSet<usize>, AHashSet<usize>)]) -> usize {
    let mut unused_cards = Vec::from_iter(0..cards.len());
    let mut used_cards = 0;

    let winnings = cards
        .iter()
        .map(|(winning, mine)| winning.intersection(mine).count())
        .collect::<Vec<_>>();

    while let Some(card) = unused_cards.pop() {
        used_cards += 1;

        for c in card + 1..card + 1 + winnings[card] {
            unused_cards.push(c);
        }
    }

    used_cards
}

fn main() {
    let input = include_str!("../../input/input04.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day04() {
    let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let input = parse(input);

    assert_eq!(part1(&input), 13);
    assert_eq!(part2(&input), 30);
}
