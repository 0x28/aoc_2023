use rand::prelude::*;

use ahash::AHashMap;

fn parse(input: &str) -> AHashMap<&str, Vec<&str>> {
    let mut graph = AHashMap::new();
    for line in input.lines() {
        let mut split = line.split(": ");
        let from = split.next().unwrap();
        let to = split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .collect::<Vec<_>>();

        for t in to.iter() {
            graph.entry(*t).or_insert(vec![]).push(from);
        }

        graph.entry(from).or_insert(vec![]).extend(to);
    }

    graph
}

fn part1(puzzle: &AHashMap<&str, Vec<&str>>) -> i64 {
    loop {
        let mut graph = AHashMap::new();

        for (k, v) in puzzle {
            graph.insert(
                k.to_string(),
                v.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            );
        }
        let mut counts = AHashMap::new();

        for v in graph.keys() {
            counts.insert(v.to_string(), 1);
        }

        // Run https://en.wikipedia.org/wiki/Karger%27s_algorithm until the last
        // remaining vertices are connected by 3 edges.
        loop {
            let vs = graph.keys().cloned().collect::<Vec<_>>();

            if vs.len() == 2 && vs.iter().all(|v| graph[v].len() == 3) {
                return counts.values().product();
            }

            if vs.len() <= 2 {
                break;
            }

            let v1 = &vs[random::<usize>() % vs.len()];
            let v1_to = graph.get(v1).unwrap().clone();
            if v1_to.is_empty() {
                continue;
            }
            let v2 = &v1_to[random::<usize>() % v1_to.len()];

            let v1_count = counts.remove(v1).unwrap();
            let v2_count = counts.remove(v2).unwrap_or(1);

            let comb = format!("{v1}+{v2}");
            counts.insert(comb.clone(), v1_count + v2_count);

            let mut new_to = vec![];
            if let Some(to) = graph.remove(v1) {
                new_to.extend(to.into_iter().filter(|n| n != v1 && n != v2));
            }

            if let Some(to) = graph.remove(v2) {
                new_to.extend(to.into_iter().filter(|n| n != v1 && n != v2));
            }

            for (_, to) in graph.iter_mut() {
                for node in to {
                    if node == v1 || node == v2 {
                        *node = comb.clone();
                    }
                }
            }

            graph.insert(comb, new_to);
        }
    }
}

fn main() {
    let input = include_str!("../../input/input25.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
}

#[test]
fn test_day25() {
    let input = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    let input = parse(input);

    assert_eq!(part1(&input), 54);
}
