fn main() {
    println!("{}", part2(include_str!("./input1.txt")));
}

// https://adventofcode.com/2023/day/2
fn part2(input: &str) -> u32 {
    return input.lines().map(process_line).sum();
}

fn process_line(s: &str) -> u32 {
    let min_cubes_for_round = s
        .split_once(":")
        .expect("should contain a ':'")
        .1
        .split(";")
        .map(|round| {
            round.split(",").fold((0, 0, 0), |mut acc, cube_in_round| {
                let count_str: String = cube_in_round.matches(|c: char| c.is_digit(10)).collect();
                let count = count_str
                    .parse::<u32>()
                    .ok()
                    .expect("cube is missing a number");

                match cube_in_round {
                    c if c.ends_with("red") => acc.0 += count,
                    c if c.ends_with("green") => acc.1 += count,
                    c if c.ends_with("blue") => acc.2 += count,
                    _ => {
                        panic!("unexpected cube color")
                    }
                }

                return acc;
            })
        })
        .fold((0, 0, 0), |mut acc, (r, g, b)| {
            acc.0 = acc.0.max(r);
            acc.1 = acc.1.max(g);
            acc.2 = acc.2.max(b);
            return acc;
        });

    return min_cubes_for_round.0 * min_cubes_for_round.1 * min_cubes_for_round.2;
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
        game1: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48,
        game2: "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12,

        game5: "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36,
    );

    process_line_test!(
        invalid_games,
        game3: "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560,
        game4: "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630,
    );

    #[test]
    fn part1_example() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}
