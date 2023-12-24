use std::ops::RangeInclusive;
use z3::ast::{Ast, Int, Real};
use z3::{Config, Context, Solver};

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

fn part1(puzzle: &[Hail], bounds: RangeInclusive<f64>) -> i64 {
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

fn part2(puzzle: &[Hail]) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);
    let x = Real::new_const(&ctx, "X");
    let y = Real::new_const(&ctx, "Y");
    let z = Real::new_const(&ctx, "Z");

    let vx = Real::new_const(&ctx, "VX");
    let vy = Real::new_const(&ctx, "VY");
    let vz = Real::new_const(&ctx, "VZ");

    // z3 is really cool :)
    for stone in puzzle.iter() {
        // real is much faster than int
        let x_other = Int::from_i64(&ctx, stone.x as i64).to_real();
        let y_other = Int::from_i64(&ctx, stone.y as i64).to_real();
        let z_other = Int::from_i64(&ctx, stone.z as i64).to_real();
        let tn = Real::fresh_const(&ctx, "T");

        let vx_other = Int::from_i64(&ctx, stone.vx as i64).to_real();
        let vy_other = Int::from_i64(&ctx, stone.vy as i64).to_real();
        let vz_other = Int::from_i64(&ctx, stone.vz as i64).to_real();

        let stone_move_x = &x + &vx * &tn;
        let stone_move_y = &y + &vy * &tn;
        let stone_move_z = &z + &vz * &tn;

        let other_move_x = &x_other + &vx_other * &tn;
        let other_move_y = &y_other + &vy_other * &tn;
        let other_move_z = &z_other + &vz_other * &tn;

        solver.assert(&stone_move_x._eq(&other_move_x));
        solver.assert(&stone_move_y._eq(&other_move_y));
        solver.assert(&stone_move_z._eq(&other_move_z));
    }

    solver.check();
    let model = solver.get_model().unwrap();

    if let (Some(x), Some(y), Some(z)) = (
        model.get_const_interp(&x).and_then(|x| x.as_real()),
        model.get_const_interp(&y).and_then(|y| y.as_real()),
        model.get_const_interp(&z).and_then(|z| z.as_real()),
    ) {
        assert!(x.1 == 1 && y.1 == 1 && z.1 == 1);
        x.0 + y.0 + z.0
    } else {
        unreachable!()
    }
}

fn main() {
    let input = include_str!("../../input/input24.txt");
    let input = parse(input);
    println!(
        "part1 = {}",
        part1(&input, 200000000000000.0..=400000000000000.0)
    );
    println!("part2 = {}", part2(&input));
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
    assert_eq!(part2(&input), 47);
}
