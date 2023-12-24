use std::ops::RangeInclusive;

#[derive(Debug)]
struct Hail {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

fn parse(input: &str) -> Vec<Hail> {
    input
        .lines()
        .map(|l| {
            let nums = l
                .split(|c: char| !c.is_ascii_digit() && c != '-')
                .flat_map(|d| d.parse().ok())
                .collect::<Vec<_>>();

            Hail {
                x: nums[0],
                y: nums[1],
                z: nums[2],
                vx: nums[3],
                vy: nums[4],
                vz: nums[5],
            }
        })
        .collect()
}

type Point = (f64, f64);

// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
fn intersection(left: &Hail, right: &Hail) -> Option<Point> {
    let a = left.vy / left.vx;
    let c = left.y - (left.x * a);

    let b = right.vy / right.vx;
    let d = right.y - (right.x * b);

    if (a - b).abs() < f64::EPSILON {
        return None;
    }

    let x = (d - c) / (a - b);
    let y = a * x + c;

    Some((x, y))
}

fn part1(puzzle: &Vec<Hail>, bounds: RangeInclusive<f64>) -> i64 {
    let mut crossings = 0;

    for (left, lhail) in puzzle.iter().enumerate() {
        for (right, rhail) in puzzle.iter().enumerate() {
            if left < right {
                let Some(inter) = intersection(rhail, lhail) else {
                    continue;
                };

                if (inter.0 - lhail.x > 0.0) == (lhail.vx > 0.0)
                    && (inter.1 - lhail.y > 0.0) == (lhail.vy > 0.0)
                    && (inter.0 - rhail.x > 0.0) == (rhail.vx > 0.0)
                    && (inter.1 - rhail.y > 0.0) == (rhail.vy > 0.0)
                    && bounds.contains(&inter.0)
                    && bounds.contains(&inter.1)
                {
                    crossings += 1;
                }
            }
        }
    }

    crossings
}

fn part2(puzzle: &Vec<Hail>) -> i64 {
    todo!()
}

fn main() {
    let input = include_str!("../../input/input24.txt");
    let input = parse(input);
    println!(
        "part1 = {}",
        part1(&input, 200000000000000.0..=400000000000000.0)
    );
    // println!("part2 = {}", part2(&input));
}

#[test]
fn test_day24() {
    let input = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    let input = parse(input);

    assert_eq!(part1(&input, 7.0..=27.0), 2);
    // assert_eq!(part2(&input), 0);
}
