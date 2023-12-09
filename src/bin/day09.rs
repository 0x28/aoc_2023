fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|w| w.parse().unwrap())
                .collect()
        })
        .collect()
}

fn calc_diffs(history: &[i64]) -> Vec<Vec<i64>> {
    let mut diffs = vec![history.to_vec()];
    let mut next = vec![];
    loop {
        for win in diffs.last().unwrap().windows(2) {
            next.push(win[1] - win[0]);
        }

        if next.iter().all(|n| *n == 0) {
            break;
        }

        diffs.push(next);
        next = vec![];
    }
    diffs
}

fn part1(histories: &[Vec<i64>]) -> i64 {
    histories
        .iter()
        .map(|history| {
            calc_diffs(history)
                .iter()
                .fold(0, |acc, d| d.last().unwrap() + acc)
        })
        .sum()
}

fn part2(histories: &[Vec<i64>]) -> i64 {
    histories
        .iter()
        .map(|history| calc_diffs(history).iter().rfold(0, |acc, d| d[0] - acc))
        .sum()
}

fn main() {
    let input = include_str!("../../input/input09.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day09() {
    let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let input = parse(input);

    assert_eq!(part1(&input), 114);
    assert_eq!(part2(&input), 2);
}
