fn main() {
    println!("{}", part1(include_str!("./input1.txt")));
}

fn part1(input: &str) -> i64 {
    let mut lines = input.lines();
    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    return *lines
        .fold(vec![vec![]], |mut acc, line| {
            let binding = line
                .split_whitespace()
                .map(|d| d.parse::<i64>().unwrap_or(0))
                .collect::<Vec<_>>();
            if let Some([dst_start, src_start, len]) = binding.get(0..3) {
                acc.last_mut()
                    .expect("accumulator to be non-empty")
                    .push((*src_start..*src_start + *len, dst_start - src_start));
            } else {
                if acc.last().unwrap().len() != 0 {
                    acc.push(vec![]);
                }
            };
            return acc;
        })
        .iter()
        .fold(seeds, |mut acc, transforms| {
            for item in acc.iter_mut() {
                *item += transforms
                    .iter()
                    .find(|(src_range, _)| src_range.contains(item))
                    .map(|(_, difference)| difference)
                    .unwrap_or(&0);
            }
            return acc;
        })
        .iter()
        .min()
        .expect("to be non empty");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = part1(
            "seeds: 79 14 55 13

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
56 93 4",
        );
        assert_eq!(result, 35);
    }
}
