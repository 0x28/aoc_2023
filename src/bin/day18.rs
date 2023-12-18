#[derive(Debug)]
struct Instruction {
    dir: char,
    length: i64,
}

fn parse1(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut s = l.split_ascii_whitespace();
            let dir = s.next().unwrap().chars().next().unwrap();
            let length = s.next().unwrap().parse().unwrap();

            Instruction { dir, length }
        })
        .collect()
}

fn parse2(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut s = l.split(|c| c == ')' || c == '#');
            let hex = s.nth(1).unwrap();
            let dir_digit = &hex[hex.len() - 1..];
            let length =
                i64::from_str_radix(&hex[0..hex.len() - 1], 16).unwrap();

            let dir = match dir_digit {
                "0" => 'R',
                "1" => 'D',
                "2" => 'L',
                "3" => 'U',
                _ => unreachable!(),
            };

            Instruction { dir, length }
        })
        .collect()
}

fn solve(instructions: &[Instruction]) -> i64 {
    let mut pos = (0, 0);
    let mut border = vec![];

    border.push(pos);

    let mut border_points = 0;
    for instr in instructions {
        border_points += instr.length;

        match instr.dir {
            'U' => pos.1 -= instr.length,
            'D' => pos.1 += instr.length,
            'L' => pos.0 -= instr.length,
            'R' => pos.0 += instr.length,
            _ => unreachable!(),
        }
        border.push(pos);
    }

    // polygon area
    // https://web.archive.org/web/20100405070507/http://valis.cs.uiuc.edu/~sariel/research/CG/compgeom/msg00831.html
    let mut area = 0;
    for i in 0..border.len() {
        let j = (i + 1) % border.len();
        area += border[i].0 * border[j].1 - border[i].1 * border[j].0;
    }

    area /= 2;

    // pick's theorem
    area + 1 - border_points / 2 + border_points
}

fn main() {
    let input = include_str!("../../input/input18.txt");
    println!("part1 = {}", solve(&parse1(input)));
    println!("part2 = {}", solve(&parse2(input)));
}

#[test]
fn test_day18() {
    let input = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    assert_eq!(solve(&parse1(input)), 62);
    assert_eq!(solve(&parse2(input)), 952408144115);
}
