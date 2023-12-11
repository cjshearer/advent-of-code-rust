use std::collections::HashSet;

fn main() {
    println!("{}", part1(include_str!("./input1.txt")));
}

// https://adventofcode.com/2023/day/4
fn part1(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| {
            let (winning_nums, our_nums) = line.split_once(":").unwrap().1.split_once("|").unwrap();
            let [winning_nums, our_nums] = [winning_nums, our_nums]
                .map(|nums| nums.split_whitespace().collect::<HashSet<_>>());
            let our_winning_nums: u32 = winning_nums
                .intersection(&our_nums)
                .collect::<HashSet<_>>()
                .len()
                .try_into()
                .unwrap();
            return match our_winning_nums {
                0 => 0,
                _ => 2_u32.pow(our_winning_nums - 1),
            };
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13);
    }
}
