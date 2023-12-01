use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

/*
let res1 = callApi1()
if res1.is_err() {
    error during calling api1
}
let res2 = callApi1()
if res2.is_err() {
    error during calling api1
}

let res1
try {
    res1 = callApi1()
} catch (
    error while calling api1
)
let res2
try {
    res2 = callApi1()
} catch (
    error while calling api1
)

 */

fn process(input: &str) -> String {
    let mut sum = 0;
    let lines = input.split("\n").collect::<Vec<&str>>();
    for line in lines {
        dbg!(&line);
        let digits = convert_line(line);
        dbg!(&digits);

        if digits.len() == 0 {
            println!("Line \"{line}\" doesnt have any digits");
            continue;
        }

        let first = digits.chars().nth(0).unwrap().to_string();
        let last = digits.chars().nth(digits.len() - 1).unwrap().to_string();

        let res = format!("{first}{last}").parse::<i32>();
        if res.is_err() {
            panic!("couldn't parse {first}{last} to int")
        }
        let num = res.unwrap();
        dbg!(&num);
        sum += num;
        dbg!(&sum);
    }
    sum.to_string()
}

fn convert_line(line: &str) -> String {
    let mut main: Vec<(usize, &str)> = Vec::new();

    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);
    map.insert("1", 1);
    map.insert("2", 2);
    map.insert("3", 3);
    map.insert("4", 4);
    map.insert("5", 5);
    map.insert("6", 6);
    map.insert("7", 7);
    map.insert("8", 8);
    map.insert("9", 9);

    for word in map.keys().into_iter() {
        let mut indices: Vec<_> = line.match_indices(word).map(|(i, _)| (i, *word)).collect();
        main.append(&mut indices);
    }

    main.sort_by(|a, b| a.0.cmp(&b.0));

    let mut converted: String = "".to_string();

    //eightwo3three
    for (_index, num) in main {
        converted += &map.get_key_value(&num).unwrap().1.to_string();
    }

    converted

    // //one2one
    // let mut one: Vec<_> = line.match_indices("one").collect();
    // let mut two: Vec<_> = line.match_indices("two").collect();
    // let mut three: Vec<_> = line.match_indices("three").collect();
    // let mut four: Vec<_> = line.match_indices("four").collect();
    // let mut five: Vec<_> = line.match_indices("five").collect();
    // let mut six: Vec<_> = line.match_indices("six").collect();
    // let mut seven: Vec<_> = line.match_indices("seven").collect();
    // let mut eight: Vec<_> = line.match_indices("eight").collect();
    // let mut nine: Vec<_> = line.match_indices("nine").collect();

    // let mut main: Vec<_> = line.match_indices("one").collect();
    // main.append(&mut one);
    // main.append(&mut two);
    // main.append(&mut three);
    // main.append(&mut four);
    // main.append(&mut five);
    // main.append(&mut six);
    // main.append(&mut seven);
    // main.append(&mut eight);
    // main.append(&mut nine);

    // dbg!(main);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(result, "281");
    }
}
