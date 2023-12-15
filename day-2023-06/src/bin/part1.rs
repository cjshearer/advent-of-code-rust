use std::iter::zip;

fn main() {
    println!("{}", part1(include_str!("./input1.txt")));
}

// https://adventofcode.com/2023/day/6
fn part1(input: &str) -> u32 {
    let [times, distances]: [_; 2] = input
        .lines()
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .split_ascii_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>()
        .try_into()
        .ok()
        .expect("there to be two input lines");

    return zip(times, distances)
        .map(|(time, distance)| {
            let discriminant = time.pow(2) - 4 * distance;
            if discriminant == 0 {
                return 0;
            }
            let rhs = (discriminant as f64).sqrt();
            let lower = (((time as f64 - rhs) / 2.0) + 1.0).floor() as u32;
            let upper = (((time as f64 + rhs) / 2.0) - 1.0).ceil() as u32;
            return 1 + upper - lower;
        })
        .product();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 288);
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(include_str!("./input1.txt")), 211904);
    }
}
