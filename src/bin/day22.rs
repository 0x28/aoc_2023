use ahash::AHashSet;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
struct Brick {
    begin: (i64, i64, i64),
    end: (i64, i64, i64),
}

impl Brick {
    fn intersects(&self, other: &Brick) -> bool {
        let self_max_x = self.begin.0.max(self.end.0);
        let other_min_x = other.begin.0.min(other.end.0);

        if self_max_x < other_min_x {
            return false;
        }

        let self_min_x = self.begin.0.min(self.end.0);
        let other_max_x = other.begin.0.max(other.end.0);

        if self_min_x > other_max_x {
            return false;
        }

        let self_max_y = self.begin.1.max(self.end.1);
        let other_min_y = other.begin.1.min(other.end.1);

        if self_max_y < other_min_y {
            return false;
        }

        let self_min_y = self.begin.1.min(self.end.1);
        let other_max_y = other.begin.1.max(other.end.1);

        if self_min_y > other_max_y {
            return false;
        }

        let self_max_z = self.begin.2.max(self.end.2);
        let self_min_z = self.begin.2.min(self.end.2);
        let other_max_z = other.begin.2.max(other.end.2);
        let other_min_z = other.begin.2.min(other.end.2);

        (self_max_z >= other_min_z) && (self_min_z <= other_max_z)
    }

    fn bottom(&self) -> i64 {
        self.begin.2.min(self.end.2)
    }
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|l| {
            let mut coords = l.split('~');
            let begin = coords.next().unwrap().split(',').collect::<Vec<_>>();
            let begin = (
                begin[0].parse().unwrap(),
                begin[1].parse().unwrap(),
                begin[2].parse().unwrap(),
            );
            let end = coords.next().unwrap().split(',').collect::<Vec<_>>();
            let end = (
                end[0].parse().unwrap(),
                end[1].parse().unwrap(),
                end[2].parse().unwrap(),
            );

            Brick { begin, end }
        })
        .collect()
}

fn does_fall(
    mut brick: Brick,
    skip: Vec<usize>,
    bricks: &[Brick],
) -> Option<Brick> {
    if brick.begin.2 == 1 || brick.end.2 == 1 {
        return None;
    }

    brick.begin.2 -= 1;
    brick.end.2 -= 1;

    for (idx, other) in bricks.iter().enumerate() {
        if !skip.contains(&idx) && brick.intersects(other) {
            return None;
        }
    }

    Some(brick)
}

fn part1(bricks: &[Brick]) -> usize {
    let mut falling_bricks = bricks.to_vec();
    falling_bricks.sort_by_key(|r| std::cmp::Reverse(r.bottom()));

    let mut fixed_bricks = Vec::with_capacity(falling_bricks.len());

    while let Some(falling_brick) = falling_bricks.pop() {
        if let Some(falling) =
            does_fall(falling_brick.clone(), vec![], &fixed_bricks)
        {
            falling_bricks.push(falling);
        } else {
            fixed_bricks.push(falling_brick);
        }
    }

    (0..fixed_bricks.len())
        .par_bridge()
        .filter(|&idx| {
            let mut causes_fall = false;
            for other in 0..fixed_bricks.len() {
                if other == idx {
                    continue;
                }

                if does_fall(
                    fixed_bricks[other].clone(),
                    vec![other, idx],
                    &fixed_bricks,
                )
                .is_some()
                {
                    causes_fall = true;
                    break;
                }
            }

            !causes_fall
        })
        .count()
}

fn part2(bricks: &[Brick]) -> usize {
    let mut falling_bricks = bricks.to_vec();
    falling_bricks.sort_by_key(|r| std::cmp::Reverse(r.bottom()));

    let mut fixed_bricks = Vec::with_capacity(falling_bricks.len());

    while let Some(falling_brick) = falling_bricks.pop() {
        if let Some(falling) =
            does_fall(falling_brick.clone(), vec![], &fixed_bricks)
        {
            falling_bricks.push(falling);
        } else {
            fixed_bricks.push(falling_brick);
        }
    }

    (0..fixed_bricks.len())
        .par_bridge()
        .map(|idx| {
            let mut causes_fall = false;
            for other in 0..fixed_bricks.len() {
                if other == idx {
                    continue;
                }

                if does_fall(
                    fixed_bricks[other].clone(),
                    vec![other, idx],
                    &fixed_bricks,
                )
                .is_some()
                {
                    causes_fall = true;
                    break;
                }
            }

            if causes_fall {
                let mut new_bricks = fixed_bricks.clone();

                let mut falling = AHashSet::new();
                let mut changed = true;
                while changed {
                    changed = false;
                    for falling_idx in 0..fixed_bricks.len() {
                        if let Some(brick) = does_fall(
                            new_bricks[falling_idx].clone(),
                            vec![idx, falling_idx],
                            &new_bricks,
                        ) {
                            new_bricks[falling_idx] = brick;
                            falling.insert(falling_idx);
                            changed = true;
                        }
                    }
                }
                falling.len()
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/input22.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day22() {
    let input = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    let input = parse(input);

    assert_eq!(part1(&input), 5);
    assert_eq!(part2(&input), 7);
}
