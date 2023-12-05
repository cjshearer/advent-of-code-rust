fn main() {
    println!("{}", part1(include_str!("./input1.txt")));
}

// https://adventofcode.com/2023/day/2
fn part1(input: &str) -> u32 {
    return input.lines().filter_map(process_line).sum();
}

fn process_line(s: &str) -> Option<u32> {
    let (id, rounds) = s.split_once(":").expect("should contain a ':'");

    for (r, g, b) in rounds.split(";").map(|round| {
        round.split(",").fold((0, 0, 0), |mut acc, cubes| {
            let count_str: String = cubes.matches(|c: char| c.is_digit(10)).collect();
            let count = count_str
                .parse::<u32>()
                .ok()
                .expect("missing number for cube");

            match cubes {
                s if s.ends_with("red") => acc.0 += count,
                s if s.ends_with("green") => acc.1 += count,
                s if s.ends_with("blue") => acc.2 += count,
                _ => {}
            }

            return acc;
        })
    }) {
        if r > 12 || g > 13 || b > 14 {
            return None;
        }
    }
    return id
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
        .ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! process_line_test {
        ($suite:ident, $($name:ident: $input:expr, $output:expr,)*) => {
            mod $suite {
                use super::*;
                $(
                    #[test]
                    fn $name() {
                        assert_eq!(process_line($input), $output);
                    }
                )*
            }
        };
    }

    process_line_test!(
        valid_games,
        game1: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Some(1),
        game2: "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Some(2),

        game5: "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Some(5),
    );

    process_line_test!(
        invalid_games,
        game3: "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", None,
        game4: "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", None,
    );

    #[test]
    fn part1_example() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8);
    }
}
