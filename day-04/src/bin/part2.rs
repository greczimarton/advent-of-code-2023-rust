/*
0-4,1
1-2,1+1
2-2,1+1+1+1
3-1,1+1+1+1
4-0,1+1
5-0,1
 */

use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut data = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let line_value = calculate_line_value(line);
        println!("adding {i}");
        data.insert(i, (line_value, 1));
    }

    let mut sum = 0;

    for i in data.clone().keys().sorted() {
        let current_card = data[i];
        println!("copy {i} from {}, {}", i + 1, i + current_card.0);
        for j in i + 1..i + current_card.0 + 1 {
            let card_to_increase = data.get_mut(&j).expect("should contain val");
            println!("increasing {j} by {}", current_card.1);
            card_to_increase.1 += current_card.1;
        }

        sum += current_card.1
    }

    dbg!(data);

    sum.to_string()
}

fn calculate_line_value(line: &str) -> usize {
    let nums: Vec<&str> = line
        .split(":")
        .nth(1)
        .expect("every line should split(:) into 2 elements")
        .split("|")
        .collect();

    let winning_numbers: Vec<u32> = nums
        .iter()
        .nth(0)
        .expect("each line's num should split(|) into 2 parts, first being winnign nums")
        .split(" ")
        .into_iter()
        .filter_map(|a| a.to_string().parse::<u32>().ok())
        .collect();

    let current_numbers: Vec<u32> = nums
        .iter()
        .nth(1)
        .expect("each line's num should split(|) into 2 parts, second being current nums")
        .split(" ")
        .into_iter()
        .filter_map(|a| a.to_string().parse::<u32>().ok())
        .collect();

    let matched_numbers: Vec<&u32> = current_numbers
        .iter()
        .clone()
        .filter(|x| winning_numbers.contains(&x))
        .collect();

    matched_numbers.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input_test.txt");
        let result = process(input);
        assert_eq!(result, "30");
    }
}
