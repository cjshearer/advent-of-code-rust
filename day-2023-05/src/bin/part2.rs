use std::ops::{AddAssign, Range, RangeBounds};

fn main() {
    println!("{}", part2(include_str!("./input1.txt")));
}

fn part2(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut seeds = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i64>>()
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<Vec<_>>();
    let transform_groups = lines.fold(vec![vec![]], |mut acc, line| {
        if let Some([dst_start, src_start, len]) = line
            .split_whitespace()
            .map(|d| d.parse().unwrap_or(0))
            .collect::<Vec<i64>>()
            .get(0..3)
        {
            acc.last_mut()
                .expect("accumulator to be non-empty")
                .push((*src_start..*src_start + *len, dst_start - src_start));
        } else {
            if acc.last().unwrap().len() != 0 {
                acc.push(vec![]);
            }
        };
        return acc;
    });

    for transform_group in transform_groups.iter() {
        let mut transformed_seeds = Vec::with_capacity(seeds.len());
        while seeds.len() != 0 {
            let seed = seeds.pop().unwrap();
            let mut transformed = false;
            for transform in transform_group {
                if let Some(transformation) = seed.transform(transform) {
                    transformed = true;
                    seeds.extend(transformation.remainder);
                    transformed_seeds.push(transformation.transformed_range);
                    break;
                }
            }
            if !transformed {
                transformed_seeds.push(seed);
            }
        }
        seeds.extend(transformed_seeds);
    }

    seeds.sort_by_key(|s| s.start);

    return seeds[0].start;
}

#[derive(Debug, PartialEq)]
struct Transformation<T> {
    transformed_range: Range<T>,
    remainder: Vec<Range<T>>,
}

trait RangeExt<T, Rhs = Self>
where
    Rhs: RangeBounds<T>,
{
    /// Returns whether the ranges overlap
    fn intersects(&self, other: &Rhs) -> bool;
    /// Returns the intersection of two ranges if they overlap, else returns None.
    fn intersection(&self, other: &Rhs) -> Option<Rhs>;
    /// Applies a linear transformation to the range, for some target range.
    fn transform(&self, transform: &(Rhs, T)) -> Option<Transformation<T>>;
}

impl<T: Ord + AddAssign + Copy> RangeExt<T> for Range<T> {
    fn intersects(&self, other: &Self) -> bool {
        self.start < other.end && self.end > other.start
    }
    fn intersection(&self, other: &Self) -> Option<Self> {
        return if self.intersects(other) {
            Some(Range {
                start: self.start.max(other.start),
                end: self.end.min(other.end),
            })
        } else {
            None
        };
    }
    fn transform(&self, (transform_range, magnitude): &(Self, T)) -> Option<Transformation<T>> {
        let Some(mut transformed_range) = self.intersection(&transform_range) else {
            return None;
        };
        transformed_range.start += *magnitude;
        transformed_range.end += *magnitude;

        let mut remainder = Vec::with_capacity(2);
        if self.start < transform_range.start {
            remainder.push(Range {
                start: self.start,
                end: transform_range.start,
            });
        }
        if self.end > transform_range.end {
            remainder.push(Range {
                start: transform_range.end,
                end: self.end,
            });
        }
        return Some(Transformation {
            transformed_range,
            remainder,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_intersects() {
        assert_eq!((0..2).intersects(&(1..2)), true);
        assert_eq!((0..2).intersects(&(0..1)), true);
        assert_eq!((0..2).intersects(&(2..3)), false);
        assert_eq!((0..2).intersects(&(-1..0)), false);
    }

    #[test]
    fn range_intersections() {
        assert_eq!((0..4).intersection(&(-3..1)), Some(0..1));
        assert_eq!((0..4).intersection(&(3..8)), Some(3..4));
        assert_eq!((0..4).intersection(&(7..8)), None);
    }

    #[test]
    fn it_transforms() {
        assert_eq!(
            (55..68).transform(&(50..98, 2)),
            Some(Transformation {
                transformed_range: 57..70,
                remainder: vec![]
            })
        );
        assert_eq!(
            (55..68).transform(&(60..89, 2)),
            Some(Transformation {
                transformed_range: 62..70,
                remainder: vec![55..60]
            })
        );
        assert_eq!(
            (55..68).transform(&(60..62, 2)),
            Some(Transformation {
                transformed_range: 62..64,
                remainder: vec![55..60, 62..68]
            })
        );
        assert_eq!(
            (55..68).transform(&(60..62, 10)),
            Some(Transformation {
                transformed_range: 70..72,
                remainder: vec![55..60, 62..68]
            })
        );
        assert_eq!((0..10).transform(&(50..98, 10)), None);
    }

    #[test]
    fn parsed_part2_example() {
        let mut seeds = vec![79..93, 55..68];

        let transform_groups = vec![
            vec![(98..100, -48), (50..98, 2)],
            vec![(15..52, -15), (52..54, -15), (0..15, 39)],
            vec![(53..61, -4), (11..53, -11), (0..7, 42), (7..11, 50)],
            vec![(18..25, 70), (25..95, -7)],
            vec![(77..100, -32), (45..64, 36), (64..77, 4)],
            vec![(69..70, -69), (0..69, 1)],
            vec![(56..93, 4), (93..97, -37)],
        ];
        for transform_group in transform_groups.iter() {
            let mut transformed_seeds = Vec::with_capacity(seeds.len());
            while seeds.len() != 0 {
                let seed = seeds.pop().unwrap();
                let mut transformed = false;
                for transform in transform_group {
                    if let Some(transformation) = seed.transform(transform) {
                        transformed = true;
                        seeds.extend(transformation.remainder);
                        transformed_seeds.push(transformation.transformed_range);
                        break;
                    }
                }
                if !transformed {
                    transformed_seeds.push(seed);
                }
            }
            seeds.extend(transformed_seeds);
        }

        seeds.sort_by_key(|s| s.start);

        assert_eq!(seeds[0].start, 46);
    }

    #[test]
    fn part2_example() {
        let result = part2(
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
        assert_eq!(result, 46);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(include_str!("./input1.txt")), 81956384);
    }
}
