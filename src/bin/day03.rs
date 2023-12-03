use ahash::AHashMap;

struct Schematic {
    symbols: AHashMap<(i64, i64), char>,
    numbers: Vec<(Vec<(i64, i64)>, i64)>,
}

fn parse(input: &str) -> Schematic {
    let mut numbers = vec![];
    let mut symbols = AHashMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut num = String::new();
        let mut coords = vec![]; // faster than a hashset in this case

        for (x, ch) in line.char_indices() {
            if ch.is_ascii_digit() {
                num.push(ch);
                coords.push((x as i64, y as i64));
            } else {
                if ch != '.' {
                    symbols.insert((x as i64, y as i64), ch);
                }
                if !num.is_empty() {
                    numbers.push((coords, num.parse::<i64>().unwrap()));
                    num = String::new();
                    coords = vec![];
                }
            }
        }

        if !num.is_empty() {
            numbers.push((coords, num.parse::<i64>().unwrap()));
        }
    }

    Schematic { symbols, numbers }
}

fn adjacent((x, y): (i64, i64)) -> [(i64, i64); 8] {
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut adj = [(0, 0); 8];

    for (i, (dir_x, dir_y)) in dirs.iter().enumerate() {
        adj[i] = (dir_x + x, dir_y + y);
    }

    adj
}

fn part1(puzzle: &Schematic) -> i64 {
    let mut sum = 0;

    for symbol_pos in puzzle.symbols.keys() {
        let adj = adjacent(*symbol_pos);
        for (coords, number) in &puzzle.numbers {
            if adj.iter().any(|c| coords.contains(c)) {
                sum += number;
            }
        }
    }

    sum
}

fn part2(puzzle: &Schematic) -> i64 {
    let mut sum = 0;

    for (symbol_pos, _) in puzzle.symbols.iter().filter(|(_, &sy)| sy == '*') {
        let adj = adjacent(*symbol_pos);
        let mut adj_nums = vec![];
        for (coords, number) in &puzzle.numbers {
            if adj.iter().any(|c| coords.contains(c)) {
                adj_nums.push(number);
            }
        }

        if let [first, second] = adj_nums[..] {
            sum += first * second;
        }
    }

    sum
}

fn main() {
    let input = include_str!("../../input/input03.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day03() {
    let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let input = parse(input);

    assert_eq!(part1(&input), 4361);
    assert_eq!(part2(&input), 467835);
}
