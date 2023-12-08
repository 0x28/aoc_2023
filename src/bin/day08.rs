use ahash::AHashMap;

#[derive(Debug)]
struct Network<'a> {
    instructions: Vec<char>,
    nodes: AHashMap<&'a str, (&'a str, &'a str)>,
}

fn parse(input: &str) -> Network {
    let mut lines = input.lines();
    let mut nodes = AHashMap::new();

    let instructions: Vec<_> = lines.next().unwrap().chars().collect();

    for line in lines {
        let node: Vec<&str> = line
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .collect();

        if let [curr, left, right] = node[..] {
            nodes.insert(curr, (left, right));
        }
    }

    Network {
        instructions,
        nodes,
    }
}

fn part1(network: &Network) -> usize {
    let mut current = "AAA";
    let mut step = 0;

    for instr in network.instructions.iter().cycle() {
        let (left, right) = network.nodes.get(current).unwrap();
        match instr {
            'L' => current = left,
            'R' => current = right,
            _ => (),
        };

        step += 1;

        if current == "ZZZ" {
            break;
        }
    }

    step
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

// https://en.wikipedia.org/wiki/Least_common_multiple
fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn part2(network: &Network) -> usize {
    let mut current: Vec<_> = network
        .nodes
        .iter()
        .filter_map(|(n, _)| if n.ends_with('A') { Some(n) } else { None })
        .collect();

    let mut first_dests = vec![];

    for (step, instr) in network.instructions.iter().cycle().enumerate() {
        for node in &mut current {
            if node.ends_with('Z') {
                first_dests.push(step);
            }
            let (left, right) = network.nodes.get(*node).unwrap();
            match instr {
                'L' => *node = left,
                'R' => *node = right,
                _ => (),
            };
        }

        if first_dests.len() == current.len() {
            break;
        }
    }

    first_dests.iter().fold(1, |a, b| lcm(a, *b))
}

fn main() {
    let input = include_str!("../../input/input08.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day08() {
    let input = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    let input = parse(input);

    assert_eq!(part1(&input), 6);

    let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let input = parse(input);
    assert_eq!(part2(&input), 6);
}
