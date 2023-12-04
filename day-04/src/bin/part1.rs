fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    for line in lines {
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

        dbg!(&winning_numbers);
        dbg!(&current_numbers);

        let matched_numbers: Vec<&u32> = current_numbers
            .iter()
            .clone()
            .filter(|x| winning_numbers.contains(&x))
            .collect();

        let matches = matched_numbers.len();

        dbg!(&matched_numbers);

        if matched_numbers.len() == 0 {
            continue;
        }

        let two: i32 = 2;
        let pow_val = u32::try_from(matches - 1).expect("should parse count-1");
        let value: i32 = two.pow(pow_val);

        dbg!(value);
        print!("============================================");
        sum += value;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input_test.txt");
        let result = process(input);
        assert_eq!(result, "13");
    }
}
