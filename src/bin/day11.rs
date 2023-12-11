#[derive(Debug)]
struct Universe {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

fn parse(input: &str) -> Universe {
    let tiles: Vec<Vec<_>> =
        input.lines().map(|l| l.chars().collect()).collect();
    let mut empty_rows = vec![];
    let mut empty_cols = vec![];

    for (y, line) in tiles.iter().enumerate() {
        if line.iter().all(|&c| c == '.') {
            empty_rows.push(y);
        }
    }

    for x in 0..tiles[0].len() {
        if (0..tiles.len()).all(|y| tiles[y][x] == '.') {
            empty_cols.push(x);
        }
    }

    let mut galaxies = vec![];

    for (y, row) in tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile != '.' {
                galaxies.push((x, y));
            }
        }
    }

    Universe {
        galaxies,
        empty_rows,
        empty_cols,
    }
}

fn solve(puzzle: &Universe, expansion: usize) -> usize {
    let mut galaxies = puzzle.galaxies.clone();
    for (galaxy_x, galaxy_y) in &mut galaxies {
        for row in puzzle.empty_rows.iter().rev() {
            if *galaxy_y > *row {
                *galaxy_y += expansion;
            }
        }
        for col in puzzle.empty_cols.iter().rev() {
            if *galaxy_x > *col {
                *galaxy_x += expansion;
            }
        }
    }

    let mut sum = 0;
    for from in galaxies.iter() {
        for to in galaxies.iter() {
            if from < to {
                let diff = from.0.abs_diff(to.0) + from.1.abs_diff(to.1);

                sum += diff
            }
        }
    }

    sum
}

fn part1(universe: &Universe) -> usize {
    solve(universe, 1)
}

fn part2(universe: &Universe) -> usize {
    solve(universe, 999999)
}

fn main() {
    let input = include_str!("../../input/input11.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day11() {
    let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let input = parse(input);

    assert_eq!(part1(&input), 374);
    assert_eq!(solve(&input, 99), 8410);
}
