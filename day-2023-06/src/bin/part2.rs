fn main() {
    println!("{}", part2(include_str!("./input1.txt")));
}

// https://adventofcode.com/2023/day/6#part2
fn part2(input: &str) -> u64 {
    let [time, distance]: [_; 2] = input
        .lines()
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .replace(" ", "")
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .ok()
        .expect("there to be two input lines");

    let discriminant = time.pow(2) - 4 * distance;
    if discriminant == 0 {
        return 0;
    }
    let rhs = (discriminant as f64).sqrt();
    let lower = (((time as f64 - rhs) / 2.0) + 1.0).floor() as u64;
    let upper = (((time as f64 + rhs) / 2.0) - 1.0).ceil() as u64;
    return 1 + upper - lower;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() {
        let result = part2(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 71503);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(include_str!("./input1.txt")), 43364472);
    }
}
