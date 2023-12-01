fn parse(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

fn part1(puzzle: &[String]) -> u32 {
    let mut sum = 0;
    for line in puzzle.iter() {
        let digits: Vec<_> =
            line.chars().filter(|c| c.is_ascii_digit()).collect();

        if let (Some(first), Some(last)) = (
            digits.first().and_then(|c| c.to_digit(10)),
            digits.last().and_then(|c| c.to_digit(10)),
        ) {
            sum += first * 10 + last;
        }
    }

    sum
}

fn replace_numbers(line: &str) -> String {
    let numbers = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut replaced = String::new();

    for i in 0..line.len() {
        let rest = &line[i..];
        let mut found = false;

        for (name, number) in numbers {
            if rest.starts_with(name) {
                replaced.push_str(number);
                found = true;
                break;
            }
        }

        if !found {
            replaced.push(line.chars().nth(i).unwrap());
        }
    }

    replaced
}

fn part2(puzzle: &[String]) -> u32 {
    let translated: Vec<_> =
        puzzle.iter().map(|s| replace_numbers(s)).collect();

    part1(&translated)
}

fn main() {
    let input = include_str!("../../input/input01.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day01() {
    let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let input = parse(input);

    assert_eq!(part1(&input), 142);

    let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let input = parse(input);
    assert_eq!(part2(&input), 281);
}
