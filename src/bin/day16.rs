use ahash::AHashSet;
use rayon::prelude::*;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn solve(tiles: &[Vec<char>], start: ((i64, i64), (i64, i64))) -> usize {
    let mut rays = vec![];
    let mut energized = AHashSet::new();

    rays.push(start);

    let mut history = AHashSet::new();

    while let Some(ray @ (mut ray_dir, ray_pos)) = rays.pop() {
        if history.contains(&ray) {
            continue;
        }
        history.insert(ray);

        if let Some(tile) = tiles
            .get(ray_pos.1 as usize)
            .and_then(|r| r.get(ray_pos.0 as usize))
        {
            let horizontal = ray_dir.0 != 0;
            energized.insert(ray_pos);

            match tile {
                '|' if horizontal => {
                    rays.push(((0, 1), (ray_pos.0, ray_pos.1 + 1)));
                    rays.push(((0, -1), (ray_pos.0, ray_pos.1 - 1)));
                    continue;
                }
                '.' | '|' => (),
                '-' if horizontal => (),
                '-' => {
                    rays.push(((1, 0), (ray_pos.0 + 1, ray_pos.1)));
                    rays.push(((-1, 0), (ray_pos.0 - 1, ray_pos.1)));
                    continue;
                }
                '\\' => {
                    ray_dir = (ray_dir.1, ray_dir.0);
                }
                '/' => {
                    ray_dir = (-ray_dir.1, -ray_dir.0);
                }
                _ => unreachable!(),
            }

            rays.push((
                ray_dir,
                (ray_pos.0 + ray_dir.0, ray_pos.1 + ray_dir.1),
            ));
        }
    }

    energized.len()
}

fn part1(tiles: &[Vec<char>]) -> usize {
    solve(tiles, ((1, 0), (0, 0)))
}

fn part2(tiles: &[Vec<char>]) -> usize {
    let height = tiles.len() as i64;
    let width = tiles[0].len() as i64;

    let mut border = vec![];

    for y in 0..height {
        border.push(((1, 0), (0, y)));
        border.push(((-1, 0), (width - 1, y)));
    }

    for x in 0..width {
        border.push(((0, 1), (x, 0)));
        border.push(((0, -1), (x, height - 1)));
    }

    border.par_iter().map(|s| solve(tiles, *s)).max().unwrap()
}

fn main() {
    let input = include_str!("../../input/input16.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day16() {
    let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    let input = parse(input);

    assert_eq!(part1(&input), 46);
    assert_eq!(part2(&input), 51);
}
