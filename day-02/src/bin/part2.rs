use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let regex_red = Regex::new(r"\d+ red").unwrap();
    let regex_green = Regex::new(r"\d+ green").unwrap();
    let regex_blue = Regex::new(r"\d+ blue").unwrap();

    let mut sum = 0;

    for line in lines {
        dbg!(line);
        let Some(min_num_of_red) = minimum_number_of_color(line, &regex_red) else {
            continue;
        };
        dbg!("red", min_num_of_red);

        let Some(min_num_of_green) = minimum_number_of_color(line, &regex_green) else {
            continue;
        };
        dbg!("green", min_num_of_green);

        let Some(min_num_of_blue) = minimum_number_of_color(line, &regex_blue) else {
            continue;
        };

        dbg!("blue", min_num_of_blue);

        let power = min_num_of_red * min_num_of_green * min_num_of_blue;

        sum += power;
    }

    sum.to_string()
}

fn minimum_number_of_color(line: &str, regex: &Regex) -> Option<u32> {
    let mut max_count_of_color = 0;

    for regex_match in regex.find_iter(line) {
        let digit = regex_match
            .as_str()
            .chars()
            .filter_map(|a| a.to_digit(10))
            .into_iter()
            .map(|i| i.to_string())
            .collect::<String>();

        let num = digit.parse::<u32>().expect("should parse to num");

        if max_count_of_color < num {
            max_count_of_color = num;
        }
    }

    return Some(max_count_of_color);
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
        assert_eq!(result, "2286");
    }
}
