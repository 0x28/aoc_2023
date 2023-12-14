use ahash::AHashMap;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn tilt_north(dish: &mut Vec<Vec<char>>) {
    for y in 0..dish.len() {
        for x in 0..dish[y].len() {
            if dish[y][x] == 'O' {
                dish[y][x] = '.';
                let mut curr_y = y;

                loop {
                    let next_y = curr_y.wrapping_sub(1);
                    match dish.get(next_y).and_then(|r| r.get(x)) {
                        Some('.') => curr_y = next_y,
                        Some(_) | None => break,
                    };
                }

                dish[curr_y][x] = 'O';
            }
        }
    }
}

fn tilt_west(dish: &mut Vec<Vec<char>>) {
    for y in 0..dish.len() {
        for x in 0..dish[y].len() {
            if dish[y][x] == 'O' {
                dish[y][x] = '.';
                let mut curr_x = x;

                loop {
                    let next_x = curr_x.wrapping_sub(1);
                    match dish.get(y).and_then(|r| r.get(next_x)) {
                        Some('.') => curr_x = next_x,
                        Some(_) | None => break,
                    };
                }

                dish[y][curr_x] = 'O';
            }
        }
    }
}

fn tilt_south(dish: &mut Vec<Vec<char>>) {
    for y in (0..dish.len()).rev() {
        for x in 0..dish[y].len() {
            if dish[y][x] == 'O' {
                dish[y][x] = '.';
                let mut curr_y = y;

                loop {
                    let next_y = curr_y + 1;
                    match dish.get(next_y).and_then(|r| r.get(x)) {
                        Some('.') => curr_y = next_y,
                        Some(_) | None => break,
                    };
                }

                dish[curr_y][x] = 'O';
            }
        }
    }
}

fn tilt_east(dish: &mut Vec<Vec<char>>) {
    for y in 0..dish.len() {
        for x in (0..dish[y].len()).rev() {
            if dish[y][x] == 'O' {
                dish[y][x] = '.';
                let mut curr_x = x;

                loop {
                    let next_x = curr_x + 1;
                    match dish.get(y).and_then(|r| r.get(next_x)) {
                        Some('.') => curr_x = next_x,
                        Some(_) | None => break,
                    };
                }

                dish[y][curr_x] = 'O';
            }
        }
    }
}

fn total_load(dish: &Vec<Vec<char>>) -> i64 {
    let mut sum = 0;
    for (y, row) in dish.iter().enumerate() {
        for tile in row.iter() {
            if *tile == 'O' {
                sum += (dish.len() - y) as i64;
            }
        }
    }
    sum
}

fn part1(puzzle: &[Vec<char>]) -> i64 {
    let mut dish = puzzle.to_vec();
    tilt_north(&mut dish);
    total_load(&dish)
}

fn part2(puzzle: &[Vec<char>]) -> i64 {
    let mut dish = puzzle.to_vec();
    let mut cache = AHashMap::new();
    let mut loop_start = None;

    for cycle in 1.. {
        tilt_north(&mut dish);
        tilt_west(&mut dish);
        tilt_south(&mut dish);
        tilt_east(&mut dish);

        let key = dish.clone();
        if let Some((start, _)) = cache.get(&key) {
            loop_start = Some(*start);
            break;
        } else {
            cache.insert(key, (cycle, total_load(&dish)));
        }
    }

    let loop_start = loop_start.unwrap();

    let mut cycle = cache
        .into_iter()
        .filter_map(
            |(_, elem @ (n, _))| {
                if n >= loop_start {
                    Some(elem)
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>();

    cycle.sort_by(|l, r| l.0.cmp(&r.0));
    let solution_idx = (1000000000 - loop_start) % cycle.len();

    cycle[solution_idx].1
}

fn main() {
    let input = include_str!("../../input/input14.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day14() {
    let input = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let input = parse(input);

    assert_eq!(part1(&input), 136);
    assert_eq!(part2(&input), 64);
}
