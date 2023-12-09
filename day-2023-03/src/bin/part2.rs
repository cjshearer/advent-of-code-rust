use std::cmp::{max, min};

fn main() {
    println!("{}", part2(include_str!("./input1.txt")));
}

// https://adventofcode.com/2023/day/3#part2
fn part2(input: &str) -> u32 {
    let mut lines = vec![""];
    lines.extend(input.lines());
    lines.push("");
    let mut gear_ratio_sum = 0;
    for window in lines.windows(3) {
        let current_line = window[1];
        for (gear_i, _) in current_line.match_indices('*') {
            let neighbor_range_lower = max(gear_i, 1) - 1;
            let neighbor_range_upper = min(gear_i + 1, current_line.len() - 1);
            let adjacent_part_numbers = window
                .iter()
                .flat_map(|&line| {
                    let mut adjacent_digits = vec![];
                    if line.is_empty() {
                        return adjacent_digits;
                    }
                    adjacent_digits.extend(
                        line[neighbor_range_lower..=neighbor_range_upper]
                            .match_indices(|c: char| c.is_digit(10))
                            .map(|(offset, _)| (offset + neighbor_range_lower, line)),
                    );
                    match adjacent_digits.len() {
                        3 => adjacent_digits.truncate(1),
                        2 => {
                            adjacent_digits.dedup_by(|(a, _), (b, _)| usize::abs_diff(*a, *b) == 1)
                        }
                        _ => {}
                    };
                    return adjacent_digits;
                })
                .collect::<Vec<_>>();
            if adjacent_part_numbers.len() != 2 {
                continue;
            }
            gear_ratio_sum +=
                adjacent_part_numbers
                    .iter()
                    .fold(1, |acc, (part_number_middle, line)| {
                        let start = line[..*part_number_middle]
                            .rfind(|c: char| !c.is_digit(10))
                            .map(|i| i + 1)
                            .unwrap_or(0);
                        let end = line[*part_number_middle..]
                            .find(|c: char| !c.is_digit(10))
                            .map(|i| i + part_number_middle)
                            .unwrap_or(line.len());
                        let part_number = line[start..end]
                            .to_string()
                            .parse::<u32>()
                            .ok()
                            .expect("should be a number");
                        return acc * part_number;
                    });
        }
    }
    return gear_ratio_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() {
        let result = part2(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 467835);
    }

    #[test]
    fn part2_my_edgecase() {
        let result = part2(
            "467
.*.
35.",
        );
        assert_eq!(result, 467 * 35)
    }

    // https://www.reddit.com/r/adventofcode/comments/189q9wv/2023_day_3_another_sample_grid_to_use/
    #[test]
    fn part2_reddit_edge_case_test() {
        let result = part2(
            "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
15.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56",
        );
        assert_eq!(result, 6756);
    }

    // https://www.reddit.com/r/adventofcode/comments/189q9wv/comment/kbsrno0/
    #[test]
    fn part2_reddit_edge_case_test_2() {
        let result = part2(
            ".......5......
..7*..*.......
...*13*.......
.......15.....",
        );
        assert_eq!(result, 442)
    }

    // https://www.reddit.com/r/adventofcode/comments/189q9wv/comment/kbtiucy/
    #[test]
    fn part2_reddit_edge_case_test_3() {
        let result = part2(
            ".......5......
..7*..*.....4*
...*13*......9
.......15.....
..............
..............
..............
..............
..............
..............
21............
...*9.........",
        );
        assert_eq!(result, 478);
    }

    // 992 * 806 + 405 * 67 + 819 * 478 + 196 * 313 + 675*861 + 276 * 155 + 692 * 985 + 207 * 160 + 80 * 31 + 938 * 233 + 75 * 997 + 285 * 521 + 181 * 606 + 946 * 437
    // https://github.com/MichaelBrunn3r/advent-of-code/blob/main/2023/day-3/src/main.rs
    #[test]
    fn part2_reddit_edge_case_test_4() {
        let result = part2(
            "............................................................................................................................................
........405...819.........514..............201....*....*806.....196......*........*............../...........@..................644....*195.
........*......*.................@.....276......538.992...........*....720.692..880........+117.266..207.........+..........................
........67....478..675*861...80..34.....*..+777..................313........*.......................*.........445.........200..*...@........
..938......75..................*.....155.................................985..#........285.....181...160.....................$.872..595.....
....*..997*....................31.............148......946...........803.......195.......*.944...*.......551+........*...867................
...233..........553.596...........436..........*..........*437..559-..*.............@..521.*......606..........519.226..........@...........
............................................................................................................................................"
            );
        assert_eq!(result, 3585594);
    }
}
