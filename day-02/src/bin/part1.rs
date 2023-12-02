fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut sum = 0;
    let lines = input.split("\n").collect::<Vec<&str>>();
    for line in lines {
        let digits: Vec<u32> = line.chars().filter_map(|a| a.to_digit(10)).collect();
        if digits.len() == 0 {
            //panic!("Line {line} doesnt have any digits")
            continue;
        }
        let first = digits[0].to_string();
        let last = digits[digits.len() - 1].to_string();

        let res = format!("{first}{last}").parse::<i32>();
        if res.is_err() {
            panic!("couldn't parse {first}{last} to int")
        }
        let num = res.unwrap();
        dbg!(&num);

        sum += num;
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
        );
        assert_eq!(result, "142");
    }
}
