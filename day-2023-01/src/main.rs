fn main() {
    println!("{}", part1(include_str!("./input1.txt")));
    println!("{}", part2(include_str!("./input1.txt")));
}

// https://adventofcode.com/2023/day/1
fn part1(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| line.matches(|c: char| c.is_digit(10)))
        .map(|mut digits| {
            let first = digits.next()?;
            Some((first, digits.last().unwrap_or(first)))
        })
        .map(|digits| digits.expect("Invalid input: each line must have an integer"))
        .map(|(first, last)| (first.to_owned() + last).parse::<u32>().ok().unwrap())
        .fold(0, |sum, curr| sum + curr);
}

// https://adventofcode.com/2023/day/1#part2
fn part2(input: &str) -> u32 {
    return input
        .lines()
        .map(decode_calibration)
        .fold(0, |sum, curr| sum + curr);
}

fn decode_calibration(input: &str) -> u32 {
    let mut result = String::new();

    let first_digit_i = input.find(|c: char| c.is_digit(10));
    for start in 0..first_digit_i.unwrap_or(input.len()) {
        if let Some(ordinal) = cardinal_as_ordinal(&input[start..], false) {
            result.push(ordinal);
            break;
        }
    }
    if result.len() == 0 {
        result.push(input.chars().nth(first_digit_i.unwrap()).unwrap());
    }

    let last_digit_i = input.rfind(|c: char| c.is_digit(10));
    for end_offset in 0..input.len() - last_digit_i.unwrap_or(0) - 1 {
        if let Some(ordinal) = cardinal_as_ordinal(&input[..input.len() - end_offset], true) {
            result.push(ordinal);
            break;
        }
    }
    if result.len() == 1 {
        result.push(input.chars().nth(last_digit_i.unwrap()).unwrap());
    }

    return result
        .parse::<u32>()
        .ok()
        .expect("input must contain an ordinal or cardinal number");
}

/// Returns the ordinal form of a cardinal value that a string starts with (or
/// ends with, if ends_with == true).
fn cardinal_as_ordinal(input: &str, ends_with: bool) -> Option<char> {
    if input.len() < 3 {
        return None;
    }
    match if ends_with {
        &input[input.len() - 3..]
    } else {
        &input[..3]
    } {
        "one" => return Some('1'),
        "two" => return Some('2'),
        "six" => return Some('6'),
        _ => {}
    }

    if input.len() < 4 {
        return None;
    }
    match if ends_with {
        &input[input.len() - 4..]
    } else {
        &input[..4]
    } {
        "four" => return Some('4'),
        "five" => return Some('5'),
        "nine" => return Some('9'),
        _ => {}
    }

    if input.len() < 5 {
        return None;
    }
    return match if ends_with {
        &input[input.len() - 5..]
    } else {
        &input[..5]
    } {
        "three" => Some('3'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, 142);
    }

    #[test]
    fn it_decodes_calibrations() {
        for (input, output) in [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
            ("eightwo", 82),
        ] {
            assert_eq!(decode_calibration(input), output);
        }
    }

    #[test]
    fn part2_example() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
