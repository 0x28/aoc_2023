use ahash::AHashSet;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn part1(map: &[Vec<char>], steps: usize) -> usize {
    let mut positions = AHashSet::new();

    positions.insert((map.len() / 2, map.len() / 2));

    for _ in 0..steps {
        let mut new_positions = AHashSet::new();

        for (x, y) in positions {
            let n = (x, y.wrapping_sub(1));
            let w = (x.wrapping_sub(1), y);
            let s = (x, y + 1);
            let e = (x + 1, y);

            for d in [n, w, s, e] {
                if let Some(t) = map.get(d.1).and_then(|r| r.get(d.0)) {
                    if *t != '#' {
                        new_positions.insert(d);
                    }
                }
            }
        }

        positions = new_positions;
    }

    positions.len()
}

fn part2(map: &[Vec<char>], steps: usize) -> usize {
    let start = (map.len() / 2, map.len() / 2);
    let mut positions = AHashSet::new();
    let height = map.len();
    let width = map[0].len();

    positions.insert((start.0 as i64, start.1 as i64));

    let mut prev = 0;
    let mut prev_diff = 0;
    let mut prev_diff_diff = 0;
    let mut next_step = 0;

    for step in 0..steps {
        let mut new_positions = AHashSet::new();

        for (x, y) in positions.iter().copied() {
            let n = (x, y - 1);
            let w = (x - 1, y);
            let s = (x, y + 1);
            let e = (x + 1, y);

            for d in [n, w, s, e] {
                let t = map[d.1.rem_euclid(height as i64) as usize]
                    [d.0.rem_euclid(width as i64) as usize];
                {
                    if t != '#' {
                        new_positions.insert(d);
                    }
                }
            }
        }

        if (step + 1) % width == steps % width {
            let diff = new_positions.len() - prev;
            let diff_diff = diff - prev_diff;
            prev_diff = diff;
            prev = new_positions.len();
            if prev_diff_diff == diff_diff {
                next_step = step + width;
                positions = new_positions;
                break;
            }
            prev_diff_diff = diff_diff;
        }
        positions = new_positions;
    }

    let mut plots = positions.len();

    if next_step == 0 {
        return plots;
    }

    // The number of plots grows quadratically after some steps.
    // diff_diff is the change of diffs between width steps.
    for _ in (next_step..steps).step_by(width) {
        let diff = prev_diff + prev_diff_diff;
        prev_diff = diff;
        plots += diff;
    }

    plots
}

fn main() {
    let input = include_str!("../../input/input21.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input, 64));
    println!("part2 = {}", part2(&input, 26501365));
}

#[test]
fn test_day21() {
    let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    let input = parse(input);

    assert_eq!(part1(&input, 6), 16);

    assert_eq!(part2(&input, 6), 16);
    assert_eq!(part2(&input, 10), 50);
    assert_eq!(part2(&input, 50), 1594);
    assert_eq!(part2(&input, 100), 6536);
    assert_eq!(part2(&input, 500), 167004);
    assert_eq!(part2(&input, 1000), 668697);
    assert_eq!(part2(&input, 5000), 16733044);
}
