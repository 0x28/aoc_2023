struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Vec<(i64, i64, i64)>>,
}

fn parse(input: &str) -> Almanac {
    let mut groups = input.split("\n\n");

    let seeds: Vec<_> = groups
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();

    let mut maps = vec![];
    for group in groups {
        let mut map = vec![];
        for line in group.lines().skip(1) {
            let nums = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            if let [a, b, c] = nums[..] {
                map.push((a, b, c));
            }
        }

        maps.push(map);
    }

    Almanac { seeds, maps }
}

fn part1(almanac: &Almanac) -> i64 {
    let mut lowest = i64::MAX;
    for seed in &almanac.seeds {
        let mut state = *seed;
        for map in &almanac.maps {
            for &(dest, source, length) in map {
                if (source..source + length).contains(&state) {
                    state = state - source + dest;
                    break;
                }
            }
        }

        lowest = lowest.min(state);
    }

    lowest
}

fn part2(almanac: &Almanac) -> i64 {
    let mut ranges: Vec<_> =
        almanac.seeds.chunks(2).map(|c| c[0]..c[0] + c[1]).collect();

    for map in &almanac.maps {
        let mut new_ranges = vec![];

        for &(dest, source, length) in map {
            let start = source;
            let end = source + length;
            let mut rest = vec![];

            while let Some(range) = ranges.pop() {
                if (start..end).contains(&range.start)
                    && (start..end).contains(&range.end)
                {
                    new_ranges.push(
                        range.start - source + dest..range.end - source + dest,
                    );
                } else if range.contains(&start) && range.contains(&(end - 1)) {
                    new_ranges.push(dest..dest + length);
                    rest.push(range.start..start);
                    rest.push(end..range.end);
                } else if range.contains(&start) {
                    new_ranges.push(dest..range.end - source + dest);
                    rest.push(range.start..start);
                } else if range.contains(&(end - 1)) {
                    new_ranges.push(range.start - source + dest..dest + length);
                    rest.push(end..range.end);
                } else {
                    rest.push(range);
                }
            }

            ranges = rest;
        }
        ranges.extend(new_ranges.into_iter());
    }

    ranges.iter().map(|r| r.start).min().unwrap()
}

fn main() {
    let input = include_str!("../../input/input05.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day05() {
    let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let input = parse(input);

    assert_eq!(part1(&input), 35);
    assert_eq!(part2(&input), 46);
}
