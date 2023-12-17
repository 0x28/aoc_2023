use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use ahash::AHashSet;

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
        .collect()
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Node {
    cost: i64,
    last_dir: (i64, i64),
    pos: (i64, i64),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn neighbors(n: &Node, map: &[Vec<i64>], min: i64, max: i64) -> Vec<Node> {
    let mut neighbors = vec![];
    let heat = |(x, y)| map.get(y as usize).and_then(|r| r.get(x as usize));
    let left_dir @ (left_x, left_y) = (n.last_dir.1, -n.last_dir.0);
    let right_dir @ (right_x, right_y) = (-n.last_dir.1, n.last_dir.0);
    let (pos_x, pos_y) = n.pos;

    for dist in min..=max {
        let left = (pos_x + left_x * dist, pos_y + left_y * dist);
        let right = (pos_x + right_x * dist, pos_y + right_y * dist);

        if heat(left).is_some() {
            let cost = (1..=dist)
                .flat_map(|d| heat((pos_x + left_x * d, pos_y + left_y * d)))
                .sum::<i64>();
            neighbors.push(Node {
                last_dir: left_dir,
                cost: n.cost + cost,
                pos: left,
            });
        }

        if heat(right).is_some() {
            let cost = (1..=dist)
                .flat_map(|d| heat((pos_x + right_x * d, pos_y + right_y * d)))
                .sum::<i64>();
            neighbors.push(Node {
                last_dir: right_dir,
                cost: n.cost + cost,
                pos: right,
            });
        }
    }

    neighbors
}

fn min_heat(map: &[Vec<i64>], min: i64, max: i64) -> Option<i64> {
    let end = ((map[0].len() - 1) as i64, (map.len() - 1) as i64);
    let mut heap = BinaryHeap::<Reverse<Node>>::new();
    let mut visited = AHashSet::new();

    heap.push(Reverse(Node {
        last_dir: (1, 0),
        cost: 0,
        pos: (0, 0),
    }));
    heap.push(Reverse(Node {
        last_dir: (0, 1),
        cost: 0,
        pos: (0, 0),
    }));

    while let Some(Reverse(node)) = heap.pop() {
        if node.pos == end {
            return Some(node.cost);
        }

        if visited.contains(&(node.pos, node.last_dir)) {
            continue; // previous iteration got here faster
        }

        visited.insert((node.pos, node.last_dir));

        for neigh in neighbors(&node, map, min, max) {
            heap.push(Reverse(neigh));
        }
    }

    None
}

fn part1(puzzle: &[Vec<i64>]) -> i64 {
    min_heat(puzzle, 1, 3).unwrap()
}

fn part2(puzzle: &[Vec<i64>]) -> i64 {
    min_heat(puzzle, 4, 10).unwrap()
}

fn main() {
    let input = include_str!("../../input/input17.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day17() {
    let input = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    let input = parse(input);

    assert_eq!(part1(&input), 102);
    assert_eq!(part2(&input), 94);
}
