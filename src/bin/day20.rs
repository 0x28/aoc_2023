use ahash::AHashMap;
use std::collections::VecDeque;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Clone, Debug)]
enum Kind<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(AHashMap<&'a str, i64>),
    Untyped,
}

#[derive(Clone, Debug)]
struct Module<'a> {
    kind: Kind<'a>,
    outputs: Vec<&'a str>,
}

fn parse(input: &str) -> AHashMap<&str, Module> {
    let mut modules = AHashMap::new();
    let mut inputs = AHashMap::new();

    for mut line in input.lines() {
        let mut kind = Kind::Untyped;
        if line.starts_with('%') {
            kind = Kind::FlipFlop(false);
            line = &line[1..];
        } else if line.starts_with('&') {
            kind = Kind::Conjunction(AHashMap::new());
            line = &line[1..];
        } else if line.starts_with("broadcaster") {
            kind = Kind::Broadcast;
        }

        let mut s = line.split(" -> ");
        let name = s.next().unwrap();
        let right = s.next().unwrap();

        let outputs = right.split(", ").collect::<Vec<_>>();

        for output in outputs.iter() {
            inputs.entry(*output).or_insert(vec![]).push(name);
        }

        modules.insert(name, Module { kind, outputs });
    }

    for (name, module) in modules.iter_mut() {
        if let Kind::Conjunction(_) = module.kind {
            module.kind = Kind::Conjunction(
                inputs[name].iter().map(|i| (*i, 0)).collect(),
            );
        }
    }

    modules
}

fn part1(modules: &AHashMap<&str, Module>) -> i64 {
    let mut lows = 0;
    let mut highs = 0;
    let mut modules = modules.clone();
    let mut inputs = VecDeque::new();

    for _ in 0..1000 {
        inputs.push_back(("broadcaster", "button", 0));
        lows += 1;

        while !inputs.is_empty() {
            let mut new_inputs = VecDeque::new();
            while let Some((dest, src, signal)) = inputs.pop_front() {
                let Some(module) = modules.get_mut(dest) else {
                    continue;
                };

                match &mut module.kind {
                    Kind::Broadcast => {
                        for output in module.outputs.iter() {
                            new_inputs.push_back((*output, dest, signal));
                        }
                    }
                    Kind::FlipFlop(state) => {
                        if signal == 0 {
                            for output in module.outputs.iter() {
                                new_inputs.push_back((
                                    output,
                                    dest,
                                    if *state { 0 } else { 1 },
                                ));
                            }
                            module.kind = Kind::FlipFlop(!*state);
                        }
                    }
                    Kind::Conjunction(inputs) => {
                        inputs.insert(src, signal);

                        if inputs.iter().all(|p| *p.1 == 1) {
                            for output in module.outputs.iter() {
                                new_inputs.push_back((output, dest, 0));
                            }
                        } else {
                            for output in module.outputs.iter() {
                                new_inputs.push_back((output, dest, 1));
                            }
                        }
                    }
                    Kind::Untyped => (),
                }
            }

            inputs = new_inputs;

            for (_, _, signal) in inputs.iter() {
                if *signal == 0 {
                    lows += 1;
                } else {
                    highs += 1;
                }
            }
        }
    }

    lows * highs
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

// https://en.wikipedia.org/wiki/Least_common_multiple
fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

fn part2(modules: &AHashMap<&str, Module>) -> i64 {
    let mut dot = Command::new("dot")
        .args(["/dev/stdin", "-Tpng", "-o", "day20.png"])
        .stdin(Stdio::piped())
        .current_dir(env::temp_dir())
        .spawn()
        .unwrap();

    let stdin = dot.stdin.as_mut().unwrap();

    write!(stdin, "digraph input {{").unwrap();
    write!(stdin, "  rx [style=filled, shape=star, color=orange];").unwrap();
    for (name, module) in modules.iter() {
        let mut num = "";
        let (shape, color) = match &module.kind {
            Kind::Broadcast => ("doublecircle", "cyan"),
            Kind::FlipFlop(_) => {
                if module.outputs.iter().any(|o| {
                    if let Kind::Conjunction(inp) = &modules[o].kind {
                        inp.len() > 1
                    } else {
                        false
                    }
                }) {
                    num = "(1)";
                } else {
                    num = "(0)";
                }
                ("diamond", "green")
            }
            Kind::Conjunction(_) => ("polygon", "red"),
            _ => unreachable!(),
        };
        write!(
            stdin,
            "  {name} [label=\"{name} {num}\", style=filled, shape={}, color={}];",
            shape, color
        )
        .unwrap();
        write!(
            stdin,
            "  {name} -> {{{}}}",
            module.outputs.to_vec().join(", ")
        )
        .unwrap();
    }
    write!(stdin, "}}").unwrap();

    dot.wait().unwrap();
    println!(
        "\nWrote graph to {}/day20.png\n",
        env::temp_dir().to_str().unwrap()
    );

    // Solved by looking a the graph (after translating it to graphviz).
    // There are four conjunctions with more than one input in the graph. Every
    // input of every conjunction module needs to be 1 (high). The conjunctions
    // are connected to a sequence of flip-flops, this is essentially a counting
    // register.

    // If the flip-flop of a counting register is connected back to the
    // conjunction we need it to be 1 to arrive at a low signal for rx. So I
    // just walked the sequence flip-flops from first to last and wrote down a 1
    // if flip-flops sends a signals to the conjunction and a 0 otherwise. Once
    // this number is reached rx can be 0 and the corresponding counting
    // register will be cleared -> it loops (lcm).

    [
        (0b111011010001), // vc
        (0b111110111011), // gf
        (0b111101010011), // db
        (0b111010011011), // qx
    ]
    .iter()
    .fold(1, |acc, n| lcm(acc, *n))
}

fn main() {
    let input = include_str!("../../input/input20.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day20() {
    let input1 = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    let input1 = parse(input1);

    let input2 = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    let input2 = parse(input2);

    assert_eq!(part1(&input1), 32000000);
    assert_eq!(part1(&input2), 11687500);
}
