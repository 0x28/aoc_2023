use std::ops::RangeInclusive;

use ahash::AHashMap;

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn resolve(&self, name: char) -> i64 {
        match name {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    operand: char,
    operator: char,
    value: i64,
    label: &'a str,
}

#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    default: &'a str,
}

#[derive(Debug)]
struct Puzzle<'a> {
    parts: Vec<Part>,
    workflows: AHashMap<&'a str, Workflow<'a>>,
}

fn parse(input: &str) -> Puzzle {
    let mut split = input.split("\n\n");

    let mut workflows = AHashMap::new();
    let mut parts = vec![];

    for line in split.next().unwrap().lines() {
        let mut comps = line
            .split(|c| c == '{' || c == '}' || c == ',')
            .filter(|c| !c.is_empty());
        let key = comps.next().unwrap();
        let mut rules = vec![];
        let mut default = "";

        for rule in comps {
            if rule.contains(':') {
                let operand = rule.as_bytes()[0] as char;
                let operator = rule.as_bytes()[1] as char;
                let mut s = rule[2..].split(':');
                let value = s.next().unwrap().parse::<i64>().unwrap();
                let label = s.next().unwrap();
                rules.push(Rule {
                    operand,
                    operator,
                    value,
                    label,
                });
            } else {
                default = rule
            }
        }

        workflows.insert(key, Workflow { rules, default });
    }

    for line in split.next().unwrap().lines() {
        let nums = line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|w| !w.is_empty())
            .map(|w| w.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        parts.push(Part {
            x: nums[0],
            m: nums[1],
            a: nums[2],
            s: nums[3],
        })
    }

    Puzzle { parts, workflows }
}

fn part1(puzzle: &Puzzle) -> i64 {
    let mut sum = 0;

    for part in puzzle.parts.iter() {
        let mut current = "in";
        while current != "A" && current != "R" {
            let workflow = &puzzle.workflows[current];
            let mut use_default = true;

            for rule in workflow.rules.iter() {
                let value = part.resolve(rule.operand);

                match rule.operator {
                    '<' => {
                        if value < rule.value {
                            current = rule.label;
                            use_default = false;
                            break;
                        }
                    }
                    '>' => {
                        if value > rule.value {
                            current = rule.label;
                            use_default = false;
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            if use_default {
                current = workflow.default;
            }
        }

        if current == "A" {
            sum += part.x + part.m + part.a + part.s;
        }
    }

    sum
}

#[derive(Debug, Clone)]
struct AcceptedRanges {
    x_range: RangeInclusive<i64>,
    m_range: RangeInclusive<i64>,
    a_range: RangeInclusive<i64>,
    s_range: RangeInclusive<i64>,
}

impl AcceptedRanges {
    fn range(&mut self, c: char) -> &mut RangeInclusive<i64> {
        match c {
            'x' => &mut self.x_range,
            'm' => &mut self.m_range,
            'a' => &mut self.a_range,
            's' => &mut self.s_range,
            _ => unreachable!(),
        }
    }

    fn replace_range(&mut self, c: char, range: &RangeInclusive<i64>) {
        *self.range(c) = range.clone();
    }
}

fn split_range(
    at: i64,
    range: &RangeInclusive<i64>,
) -> Option<(RangeInclusive<i64>, RangeInclusive<i64>)> {
    if range.contains(&at) {
        Some((*range.start()..=at, at + 1..=*range.end()))
    } else {
        None
    }
}

fn solve<'a>(
    puzzle: &'a Puzzle,
    current: &'a str,
    accepted: &AcceptedRanges,
) -> i64 {
    // Luckily there are no overlapping ranges.
    if current == "A" {
        return [
            &accepted.x_range,
            &accepted.m_range,
            &accepted.a_range,
            &accepted.s_range,
        ]
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .product();
    } else if current == "R" {
        return 0;
    }

    let workflow = &puzzle.workflows[current];
    let mut accepted = accepted.clone();
    let mut sum = 0;

    for rule in workflow.rules.iter() {
        match rule.operator {
            '<' => {
                if let Some((l, r)) =
                    split_range(rule.value - 1, accepted.range(rule.operand))
                {
                    accepted.replace_range(rule.operand, &l);
                    sum += solve(puzzle, rule.label, &accepted);
                    accepted.replace_range(rule.operand, &r);
                } else if rule.value >= *accepted.range(rule.operand).end() {
                    sum += solve(puzzle, rule.label, &accepted);
                }
            }
            '>' => {
                if let Some((l, r)) =
                    split_range(rule.value, accepted.range(rule.operand))
                {
                    accepted.replace_range(rule.operand, &r);
                    sum += solve(puzzle, rule.label, &accepted);
                    accepted.replace_range(rule.operand, &l);
                } else if rule.value < *accepted.range(rule.operand).start() {
                    sum += solve(puzzle, rule.label, &accepted);
                }
            }
            _ => unreachable!(),
        }
    }

    sum += solve(puzzle, workflow.default, &accepted);

    sum
}

fn part2(puzzle: &Puzzle) -> i64 {
    let ranges = AcceptedRanges {
        x_range: 1..=4000,
        m_range: 1..=4000,
        a_range: 1..=4000,
        s_range: 1..=4000,
    };

    solve(puzzle, "in", &ranges)
}

fn main() {
    let input = include_str!("../../input/input19.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day19() {
    let input = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    let input = parse(input);

    assert_eq!(part1(&input), 19114);
    assert_eq!(part2(&input), 167409079868000);

    let input = "\
alpha{a>3500:A,a<501:A,R}
in{alpha}

";
    let input = parse(input);

    assert_eq!(part2(&input), 4000 * 4000 * 4000 * 1000);
}
