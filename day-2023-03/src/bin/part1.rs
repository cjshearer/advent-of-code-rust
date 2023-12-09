fn main() {
    println!("{}", part1(include_str!("./input1.txt")));
}

// https://adventofcode.com/2023/day/3
fn part1(input: &str) -> u32 {
    let mut lines = input
        .lines()
        .map(|l| l.char_indices().collect::<Vec<_>>())
        .peekable();
    let mut previous_line: Vec<(usize, char)> = Vec::new();
    let mut output = 0;
    while let Some(current_line) = lines.next() {
        let next_line = lines.peek();
        let mut str_number = String::new();
        let mut valid_number = false;
        for &(i, c) in &current_line {
            if c.is_digit(10) {
                str_number.push(c);
                // mark number as valid, or maintain current validity of number
                valid_number = valid_number
                    || (0..8).any(|neighbor| {
                        (match neighbor {
                            0 if i >= 1 => previous_line.get(i - 1),
                            1 => previous_line.get(i),
                            2 => previous_line.get(i + 1),
                            3 if i >= 1 => current_line.get(i - 1),
                            4 => current_line.get(i + 1),
                            5 if next_line.is_some() && i >= 1 => next_line.unwrap().get(i - 1),
                            6 if next_line.is_some() => next_line.unwrap().get(i),
                            7 if next_line.is_some() => next_line.unwrap().get(i + 1),
                            _ => None,
                        })
                        .is_some_and(|(_, neighbor)| !neighbor.is_digit(10) && neighbor != &'.')
                    });
            } else if !valid_number {
                str_number.clear();
            }
            // flush number
            if valid_number && (!c.is_digit(10) || i == current_line.len() - 1) {
                output += str_number.parse::<u32>().ok().expect("should be digit");
                valid_number = false;
                str_number.clear();
            }
        }

        previous_line = current_line;
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = part1(
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
        assert_eq!(result, 4361);
    }
}
