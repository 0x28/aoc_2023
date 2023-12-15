fn parse(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |curr, c| ((curr + c as usize) * 17) % 256)
}

fn part1(puzzle: &[&str]) -> usize {
    puzzle.iter().map(|s| hash(s)).sum()
}

fn part2(puzzle: &[&str]) -> usize {
    const EMPTY: Vec<(&str, usize)> = vec![];
    let mut map: [Vec<(&str, usize)>; 256] = [EMPTY; 256];

    for instr in puzzle {
        if instr.contains('=') {
            let mut split = instr.split('=');
            let key = split.next().unwrap();
            let value = split.next().unwrap().parse::<usize>().unwrap();

            let hash = hash(key);

            if let Some(lbox) = map.get_mut(hash) {
                let mut exists = false;
                for (k, v) in lbox {
                    if *k == key {
                        *v = value;
                        exists = true;
                        break;
                    }
                }

                if !exists {
                    map[hash].push((key, value));
                }
            }
        } else if instr.contains('-') {
            let mut split = instr.split('-');
            let key = split.next().unwrap();
            let hash = hash(key);

            if let Some(lbox) = map.get_mut(hash) {
                if let Some(pos) = lbox.iter().position(|(k, _)| *k == key) {
                    lbox.remove(pos);
                }
            }
        }
    }

    map.iter()
        .enumerate()
        .filter(|(_, b)| !b.is_empty())
        .map(|(box_nr, lbox)| {
            lbox.iter()
                .enumerate()
                .map(|(slot, &(_, len))| (box_nr + 1) * (slot + 1) * len)
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/input15.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day15() {
    let input = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let input = parse(input);

    assert_eq!(part1(&input), 1320);
    assert_eq!(part2(&input), 145);
}
