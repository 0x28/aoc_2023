fn parse1(input: &str) -> Vec<(i64, i64)> {
    let nums: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            l.split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    nums[0]
        .iter()
        .copied()
        .zip(nums[1].iter().copied())
        .collect()
}

fn parse2(input: &str) -> (i64, i64) {
    let nums: Vec<i64> = input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect();

    (nums[0], nums[1])
}

fn possible_records(time: f64, record: f64) -> i64 {
    // applied quadratic formula
    let zero_point = |sig| {
        // 0.01 because we want to be better than the record
        (-time + sig * f64::sqrt(time * time - 4.0 * (record + 0.01))) / -2.0
    };

    (zero_point(-1.0).ceil() - zero_point(1.0).ceil()) as i64
}

fn part1(puzzle: &[(i64, i64)]) -> i64 {
    puzzle
        .iter()
        .map(|&(time, record)| possible_records(time as f64, record as f64))
        .product()
}

fn part2((time, record): (i64, i64)) -> i64 {
    possible_records(time as f64, record as f64)
}

fn main() {
    let input = "\
Time:        57     72     69     92
Distance:   291   1172   1176   2026";
    println!("part1 = {}", part1(&parse1(input)));
    println!("part2 = {}", part2(parse2(input)));
}

#[test]
fn test_day06() {
    let input = "\
Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(part1(&parse1(input)), 288);
    assert_eq!(part2(parse2(input)), 71503);
}
