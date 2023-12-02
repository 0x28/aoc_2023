use std::iter::Peekable;
use std::str::{Chars, FromStr};

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Vec<(i32, String)>>,
}

fn parse(input: &str) -> Vec<Game> {
    input.lines().map(parse_line).collect()
}

fn parse_while<T, P>(chars: &mut Peekable<Chars<'_>>, pred: P) -> Option<T>
where
    T: FromStr,
    P: Fn(&char) -> bool,
{
    let mut token = String::new();

    while let Some(c) = chars.peek() {
        if pred(c) {
            token.push(*c);
            chars.next();
        } else {
            break;
        }
    }

    token.parse::<T>().ok()
}

fn parse_line(line: &str) -> Game {
    let mut chars = line.chars().peekable();
    let mut id = 0;

    while let Some(c) = chars.peek() {
        if c.is_ascii_digit() {
            id = parse_while(&mut chars, char::is_ascii_digit).unwrap();
            break;
        }
        chars.next();
    }

    let mut game = Game { id, sets: vec![] };
    let mut set = vec![];
    let mut current_number: Option<i32> = None;

    while let Some(c) = chars.peek() {
        match c {
            d if d.is_ascii_digit() => {
                current_number = parse_while(&mut chars, char::is_ascii_digit);
            }
            l if l.is_alphabetic() => {
                set.push((
                    current_number.unwrap(),
                    parse_while(&mut chars, |c| c.is_alphabetic()).unwrap(),
                ));
            }
            ';' => {
                game.sets.push(set);
                set = vec![];
                chars.next();
            }
            _ => {
                chars.next();
            }
        }
    }

    game.sets.push(set);

    game
}

fn max_colors(game: &Game) -> (i32, i32, i32) {
    let mut max = (0, 0, 0);
    for set in &game.sets {
        for (num, color) in set {
            match color.as_ref() {
                "red" => max.0 = max.0.max(*num),
                "green" => max.1 = max.1.max(*num),
                "blue" => max.2 = max.2.max(*num),
                _ => (),
            }
        }
    }

    max
}

fn part1(puzzle: &[Game]) -> i32 {
    let mut sum = 0;
    for game in puzzle {
        let (red, green, blue) = max_colors(game);
        if red <= 12 && green <= 13 && blue <= 14 {
            sum += game.id;
        }
    }

    sum
}

fn part2(puzzle: &[Game]) -> i32 {
    let mut sum = 0;

    for game in puzzle.iter() {
        let (red, green, blue) = max_colors(game);
        sum += red * green * blue;
    }

    sum
}

fn main() {
    let input = include_str!("../../input/input02.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day02() {
    let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let input = parse(input);

    assert_eq!(part1(&input), 8);
    assert_eq!(part2(&input), 2286);
}
