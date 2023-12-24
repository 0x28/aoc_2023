fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

type Neighbors = fn(&[Vec<char>], (usize, usize)) -> Vec<(usize, usize)>;

fn neighbors1(maze: &[Vec<char>], pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut posistions = vec![];

    for p @ (x, y) in [
        (pos.0, pos.1 + 1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1.wrapping_sub(1)),
        (pos.0.wrapping_sub(1), pos.1),
    ] {
        match maze.get(y).and_then(|r| r.get(x)) {
            Some('.') => posistions.push(p),
            Some('>') if x > pos.0 => posistions.push(p),
            Some('v') if y > pos.1 => posistions.push(p),
            Some('<') if x < pos.0 => posistions.push(p),
            Some('^') if y < pos.1 => posistions.push(p),
            _ => (),
        }
    }

    posistions
}

fn neighbors2(maze: &[Vec<char>], pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut posistions = vec![];

    for p @ (x, y) in [
        (pos.0, pos.1 + 1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1.wrapping_sub(1)),
        (pos.0.wrapping_sub(1), pos.1),
    ] {
        match maze.get(y).and_then(|r| r.get(x)) {
            Some(t) if *t != '#' => posistions.push(p),
            _ => (),
        }
    }

    posistions
}

fn walk(
    maze: &mut [Vec<char>],
    pos: (usize, usize),
    neighbors: Neighbors,
) -> i64 {
    if pos == (maze[0].len() - 2, maze.len() - 1) {
        return 0;
    }

    let prev = maze[pos.1][pos.0];
    maze[pos.1][pos.0] = '#';
    let mut max_dist = i64::MIN;
    for neighbor in neighbors(maze, pos) {
        max_dist = max_dist.max(1 + walk(maze, neighbor, neighbors));
    }
    maze[pos.1][pos.0] = prev;

    max_dist
}

fn part1(maze: &[Vec<char>]) -> i64 {
    let mut maze = maze.to_vec();
    walk(&mut maze, (1, 0), neighbors1)
}

fn part2(maze: &[Vec<char>]) -> i64 {
    let mut maze = maze.to_vec();
    // just brute force (takes some minutes)
    walk(&mut maze, (1, 0), neighbors2)
}

fn main() {
    let input = include_str!("../../input/input23.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day23() {
    let input = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    let input = parse(input);

    assert_eq!(part1(&input), 94);
    assert_eq!(part2(&input), 154);
}
