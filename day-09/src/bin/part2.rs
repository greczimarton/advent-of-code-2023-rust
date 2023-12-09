//1772145754 GOOD

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut lines = parse_input(&input);

    let mut new_values = vec![];

    for line in lines.iter_mut() {
        let result_of_line = get_new_value_of_line(line);
        new_values.push(result_of_line);
    }

    dbg!(&new_values);

    let result: i64 = new_values.iter().sum();

    result.to_string()
}

fn get_new_value_of_line(line: &mut Vec<i64>) -> i64 {
    line.reverse();
    let mut diffs: Vec<Vec<i64>> = vec![];
    diffs.push(line.clone());

    let mut row_count = 1;

    loop {
        diffs.push(vec![]);
        let first_diff = diffs[row_count - 1][1] - diffs[row_count - 1][0];
        diffs[row_count].push(first_diff);

        for i in 1..diffs[row_count - 1].len() {
            if i == 0 || i == &diffs[row_count - 1].len() - 1 {
                continue;
            }

            let diff = diffs[row_count - 1][i + 1] - diffs[row_count - 1][i];
            diffs[row_count].push(diff);
        }

        if diffs[row_count].iter().all(|diff| diff == &0) {
            let result = calculate_result_for_line(&mut diffs, row_count);
            dbg!(diffs);
            return result;
        }

        // if row_count > 10 {
        //     panic!("too many rows:(");
        // }

        row_count += 1;
    }
}

fn calculate_result_for_line(diffs: &mut Vec<Vec<i64>>, current_row_count: usize) -> i64 {
    let mut row: usize = current_row_count - 1;

    let mut last_diffs = vec![];

    let mut new_val = 0;

    dbg!(new_val);

    loop {
        new_val = new_val + diffs[row].last().expect("row should have val");

        // println!("new val in row {}: {}", row, new_val);

        last_diffs.push(new_val);
        if row == 0 {
            break;
        }
        row -= 1;
    }

    dbg!(&last_diffs);
    return new_val;
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|digit| {
                    digit
                        .parse::<i64>()
                        .expect("should parse input digit tu u64")
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = include_str!("./input_test.txt");
        let parsed = parse_input(input);
        assert_eq!(parsed[0], [0, 3, 6, 9, 12, 15]);
        assert_eq!(parsed[1], [1, 3, 6, 10, 15, 21]);
        assert_eq!(parsed[2], [10, 13, 16, 21, 30, 45]);
    }

    #[test]
    fn it_works() {
        let input = include_str!("./input_test.txt");
        let result = process(input);
        assert_eq!(result, "2");
    }
}
