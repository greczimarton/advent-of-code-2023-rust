// 0mm/ms
// 1ms = +1mm/ms
// v = s/t
// s = v * t
// s = (T.hold * 1[mm/ms]) * (7 - T.Hold)[ms]
// first example:
// 9 < x * (7 - x)
// 9 < 7x - xˆ2
// 0 < -xˆ2 + 7x - 9

// 200 < 30x - xˆ2

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let data: Vec<i64> = input
        .lines()
        .map(|line| {
            line.split(':')
                .nth(1)
                .unwrap()
                .replace(" ", "")
                .parse::<i64>()
                .expect("should parse input values to 64")
        })
        .collect();

    let mut result = 1;

    let time = data[0];
    let record = data[1];

    // -xˆ2 + time * x - record
    let top_left = -time as f64;
    let top_right = ((time.pow(2) - (4 * record)) as f64).sqrt();
    let mut min = (top_left + top_right) / -2_f64;
    let mut max = (top_left - top_right) / -2_f64;

    if min.fract() == 0.0 {
        min += 1_f64;
    } else {
        min = min.ceil();
    }

    if max.fract() == 0.0 {
        max -= 1_f64;
    } else {
        max = max.floor();
    }

    dbg!(min);
    dbg!(max);
    let number_of_ways_to_win = (max - min + 1_f64) as u64;
    dbg!(number_of_ways_to_win);

    result *= number_of_ways_to_win;

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input_test.txt");
        let result = process(input);
        assert_eq!(result, "71503");
    }
}
