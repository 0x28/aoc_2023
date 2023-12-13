use ahash::AHashMap;
use rayon::prelude::*;

#[derive(Debug)]
struct Springs {
    row: Vec<char>,
    condition: Vec<i64>,
}

fn parse(input: &str) -> Vec<Springs> {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_ascii_whitespace();
            let row = words.next().unwrap().chars().collect();
            let condition = words
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();

            Springs { row, condition }
        })
        .collect()
}

fn prefix_valid<'a, 'b>(
    prefix: &'a mut [char],
    cond: &'b [i64],
) -> Option<(&'a mut [char], &'b [i64])> {
    let mut num = 0;
    for (n, c) in prefix.iter().enumerate().skip_while(|(_, &c)| c == '.') {
        match c {
            '#' => num += 1,
            '.' => {
                if cond.first() == Some(&num) {
                    return Some((&mut prefix[n..], &cond[1..]));
                } else {
                    return None;
                }
            }
            '?' => {
                if let Some(c) = cond.first() {
                    if *c >= num {
                        return Some((prefix, cond));
                    } else {
                        return None;
                    }
                } else {
                    return Some((prefix, cond));
                }
            }
            _ => unreachable!(),
        }
    }

    if let Some(c) = cond.first() {
        if *c == num {
            Some((prefix, cond))
        } else {
            None
        }
    } else {
        Some((&mut [], &[]))
    }
}

fn arrangements(
    cache: &mut AHashMap<(String, Vec<i64>), i64>,
    row: &mut [char],
    cond: &[i64],
) -> i64 {
    let key = (row.iter().collect::<String>(), cond.to_vec());

    if let Some(v) = cache.get(&key) {
        return *v;
    }

    let Some((rest, cond_rest)) = prefix_valid(row, cond) else {
        return 0;
    };

    for i in 0..rest.len() {
        if rest[i] == '?' {
            rest[i] = '#';
            let mut sum = arrangements(cache, rest, cond_rest);
            rest[i] = '.';
            sum += arrangements(cache, rest, cond_rest);
            rest[i] = '?';

            cache.insert(key, sum);

            return sum;
        }
    }

    let nums: Vec<i64> = row
        .split(|c| *c == '.')
        .filter(|w| !w.is_empty())
        .map(|w| w.len() as i64)
        .collect();

    let res = if nums == cond { 1 } else { 0 };

    cache.insert(key, res);

    res
}

fn part1(puzzle: &[Springs]) -> i64 {
    let mut cache = AHashMap::new();

    puzzle
        .iter()
        .map(|springs| {
            let mut row = springs.row.clone();

            arrangements(&mut cache, &mut row, &springs.condition)
        })
        .sum()
}

fn part2(puzzle: &[Springs]) -> i64 {
    puzzle
        .par_iter()
        .map(|p| {
            let mut row =
                (0..5).map(|_| &p.row[..]).collect::<Vec<_>>().join(&'?');
            let cond: Vec<_> = (0..5).map(|_| &p.condition[..]).collect();
            let cond = cond.concat();
            let mut cache = AHashMap::new();

            arrangements(&mut cache, &mut row, &cond)
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/input12.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day12() {
    let input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let input = parse(input);

    assert_eq!(part1(&input), 21);
    assert_eq!(part2(&input), 525152);
}
