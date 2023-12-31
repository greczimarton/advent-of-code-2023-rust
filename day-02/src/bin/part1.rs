use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let max_red: u32 = 12;
    let regex_red = Regex::new(r"\d+ red").unwrap();

    let max_green: u32 = 13;
    let regex_green = Regex::new(r"\d+ green").unwrap();

    let max_blue: u32 = 14;
    let regex_blue = Regex::new(r"\d+ blue").unwrap();

    let mut sum = 0;

    for line in lines {
        dbg!(line);
        let too_many_red = is_color_too_many(line, &regex_red, &max_red);
        if too_many_red {
            continue;
        }

        let too_many_green = is_color_too_many(line, &regex_green, &max_green);
        if too_many_green {
            continue;
        }

        let too_many_blue = is_color_too_many(line, &regex_blue, &max_blue);
        if too_many_blue {
            continue;
        }

        let game_id = line
            .split(":")
            .nth(0)
            .expect("should have something like 'game 5' before ':'")
            .split(" ")
            .nth(1)
            .expect("should have 2 elements game and id")
            .to_string();

        dbg!(&game_id);

        let game_id_num = game_id.parse::<u32>().expect("should parse id to u32");

        sum += game_id_num;
    }

    sum.to_string()
}

fn is_color_too_many(line: &str, regex: &Regex, max_count: &u32) -> bool {
    for regex_match in regex.find_iter(line) {
        let digit = regex_match
            .as_str()
            .chars()
            .filter_map(|a| a.to_digit(10))
            .into_iter()
            .map(|i| i.to_string())
            .collect::<String>();

        let num = digit.parse::<u32>().expect("should parse to num");

        if max_count < &num {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "8");
    }
}
