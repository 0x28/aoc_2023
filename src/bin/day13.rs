#[derive(Debug, Clone)]
struct Pattern {
    tiles: Vec<Vec<char>>,
}

fn parse(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|p| Pattern {
            tiles: p.lines().map(|l| l.chars().collect()).collect(),
        })
        .collect()
}

fn horizontal_mirrors(pattern: &Pattern) -> Vec<i64> {
    let width = pattern.tiles[0].len();
    let height = pattern.tiles.len();

    let mut res = vec![];

    for div in 1..width {
        if (0..width).all(|dist| {
            (0..height).all(|y| {
                let left = pattern.tiles[y].get(div.wrapping_sub(1 + dist));
                let right = pattern.tiles[y].get(div + dist);

                if let (Some(left), Some(right)) = (left, right) {
                    left == right
                } else {
                    true
                }
            })
        }) {
            res.push(div as i64);
        }
    }

    res
}

fn vertical_mirrors(pattern: &Pattern) -> Vec<i64> {
    let width = pattern.tiles[0].len();
    let height = pattern.tiles.len();

    let mut res = vec![];

    for div in 1..height {
        if (0..height).all(|dist| {
            (0..width).all(|x| {
                let top = pattern
                    .tiles
                    .get(div.wrapping_sub(1 + dist))
                    .map(|r| &r[x]);
                let bottom = pattern.tiles.get(div + dist).map(|r| &r[x]);

                if let (Some(left), Some(right)) = (top, bottom) {
                    left == right
                } else {
                    true
                }
            })
        }) {
            res.push(div as i64);
        }
    }

    res
}

fn part1(puzzle: &[Pattern]) -> i64 {
    puzzle
        .iter()
        .map(|p| {
            horizontal_mirrors(p).first().copied().unwrap_or_default()
                + 100 * vertical_mirrors(p).first().copied().unwrap_or_default()
        })
        .sum()
}

fn part2(puzzle: &[Pattern]) -> i64 {
    puzzle
        .iter()
        .map(|p| {
            let mut cleaned = p.clone();
            let hori_dirt = horizontal_mirrors(p).first().copied();
            let vert_dirt = vertical_mirrors(p).first().copied();

            for y in 0..cleaned.tiles.len() {
                for x in 0..cleaned.tiles[y].len() {
                    toggle_tile(&mut cleaned, y, x);

                    let hori_clean = horizontal_mirrors(&cleaned)
                        .iter()
                        .find(|&&m| Some(m) != hori_dirt)
                        .copied();

                    let vert_clean = vertical_mirrors(&cleaned)
                        .iter()
                        .find(|&&m| Some(m) != vert_dirt)
                        .copied();

                    if vert_clean.is_some() || hori_clean.is_some() {
                        return hori_clean.unwrap_or_default()
                            + 100 * vert_clean.unwrap_or_default();
                    }

                    toggle_tile(&mut cleaned, y, x)
                }
            }

            unreachable!()
        })
        .sum()
}

fn toggle_tile(cleaned: &mut Pattern, y: usize, x: usize) {
    if cleaned.tiles[y][x] == '.' {
        cleaned.tiles[y][x] = '#';
    } else {
        cleaned.tiles[y][x] = '.';
    }
}

fn main() {
    let input = include_str!("../../input/input13.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day13() {
    let input = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let input = parse(input);

    assert_eq!(part1(&input), 405);
    assert_eq!(part2(&input), 400);
}
