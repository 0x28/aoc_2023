use ahash::AHashSet;

#[derive(Debug)]
struct Maze {
    start: (i64, i64),
    tiles: Vec<Vec<char>>,
}

impl Maze {
    fn at(&self, x: i64, y: i64) -> Option<char> {
        if x < 0 || y < 0 {
            None
        } else {
            let (x, y) = (x as usize, y as usize);

            self.tiles.get(y).and_then(|r| r.get(x)).copied()
        }
    }
}

fn parse(input: &str) -> Maze {
    let mut tiles = vec![];
    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, tile) in line.chars().enumerate() {
            if tile == 'S' {
                start = (x as i64, y as i64);
            }

            row.push(tile);
        }

        tiles.push(row);
    }

    Maze { tiles, start }
}

fn connects_dir(dir: (i64, i64), pipe: char) -> bool {
    let north_open = ['|', 'L', 'J'];
    let east_open = ['-', 'L', 'F'];
    let south_open = ['|', 'F', '7'];
    let west_open = ['-', 'J', '7'];

    if pipe == 'S' {
        return true;
    }

    match dir {
        (0, -1) => north_open.contains(&pipe),
        (0, 1) => south_open.contains(&pipe),
        (1, 0) => east_open.contains(&pipe),
        (-1, 0) => west_open.contains(&pipe),
        _ => false,
    }
}

fn step(
    maze: &Maze,
    (curr_x, curr_y): (i64, i64),
    visited: &mut AHashSet<(i64, i64)>,
) -> Option<(i64, i64)> {
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let curr_tile = maze.at(curr_x, curr_y).unwrap();

    for dir in dirs {
        let pos @ (x, y) = (curr_x + dir.0, curr_y + dir.1);

        if let Some(tile) = maze.at(x, y) {
            if visited.contains(&pos) {
                continue;
            }

            if connects_dir(dir, curr_tile)
                && connects_dir((-dir.0, -dir.1), tile)
            {
                visited.insert(pos);
                return Some(pos);
            }
        }
    }

    None
}

fn part1(maze: &Maze) -> usize {
    let mut pos = maze.start;
    let mut visited = AHashSet::new();
    let mut steps = 1;

    visited.insert(pos);

    while let Some(next_pos) = step(maze, pos, &mut visited) {
        pos = next_pos;
        steps += 1;
    }

    steps / 2
}

// https://wrfranklin.org/Research/Short_Notes/pnpoly.html
fn inside_polygon(vertices: &[(i64, i64)], point: &(i64, i64)) -> bool {
    let mut prev = vertices.len() - 1;
    let mut inside = false;

    for curr in 0..vertices.len() {
        if (vertices[curr].1 > point.1) != (vertices[prev].1 > point.1)
            && (point.0
                < (vertices[prev].0 - vertices[curr].0)
                    * (point.1 - vertices[curr].1)
                    / (vertices[prev].1 - vertices[curr].1)
                    + vertices[curr].0)
        {
            inside = !inside;
        }

        prev = curr;
    }

    inside
}

fn part2(maze: &Maze) -> i64 {
    let mut pos = maze.start;
    let mut visited = AHashSet::new();
    let mut steps = vec![pos];
    let mut count = 0;

    visited.insert(pos);

    while let Some(next_pos) = step(maze, pos, &mut visited) {
        pos = next_pos;
        steps.push(pos);
    }

    for y in 0..maze.tiles.len() {
        for x in 0..maze.tiles[y].len() {
            let pos = (x as i64, y as i64);
            if !visited.contains(&pos) && inside_polygon(&steps, &pos) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("../../input/input10.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day10() {
    let input = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    let input = parse(input);

    assert_eq!(part1(&input), 8);

    let input = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    let input = parse(input);

    assert_eq!(part2(&input), 8);
}
