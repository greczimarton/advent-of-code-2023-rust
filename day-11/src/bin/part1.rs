use std::char;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (empty_column, empty_rows, points) = parse_input(&input);

    let mut sum = 0;

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let point_a = points[i];
            let point_b = points[j];

            let extra_col_count = count_extra_col(point_a, point_b, &empty_column);
            let extra_row_count = count_extra_row(point_a, point_b, &empty_rows);

            let distance = (point_a.row as isize - point_b.row as isize).abs()
                + (point_a.column as isize - point_b.column as isize).abs()
                + extra_col_count as isize
                + extra_row_count as isize;

            sum += distance;
        }
    }

    sum.to_string()
}

fn count_extra_col(a: Point, b: Point, empty: &Vec<usize>) -> usize {
    let (min_col, max_col) = if a.column < b.column {
        (a.column, b.column)
    } else {
        (b.column, a.column)
    };

    let temp_col_range = min_col + 1..max_col;
    empty.iter().filter(|i| temp_col_range.contains(i)).count()
}

fn count_extra_row(a: Point, b: Point, empty: &Vec<usize>) -> usize {
    let (min_row, max_row) = if a.row < b.row {
        (a.row, b.row)
    } else {
        (b.row, a.row)
    };
    let temp_row_range = min_row + 1..max_row;
    empty.iter().filter(|i| temp_row_range.contains(i)).count()
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>, Vec<Point>) {
    const GALAXY_CHAR: char = '#';
    let mut points: Vec<Point> = vec![];

    let lines: Vec<&str> = input.lines().collect();

    let mut empty_columns: Vec<bool> = vec![true; lines[0].len()];
    let mut empty_rows: Vec<bool> = vec![true; lines.len()];

    for (row, line) in lines.iter().enumerate() {
        for (column, char) in line.chars().enumerate() {
            if char == GALAXY_CHAR {
                empty_columns[column] = false;
                empty_rows[row] = false;
                points.push(Point {
                    row: row,
                    column: column,
                })
            }
        }
    }

    let empty_col = empty_columns
        .iter()
        .enumerate()
        .filter(|(_, &is_empty)| is_empty == true)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    let empty_row = empty_rows
        .iter()
        .enumerate()
        .filter(|(_, &is_not_empty)| is_not_empty == true)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    (empty_col, empty_row, points)
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Point {
    row: usize,
    column: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = include_str!("./input_test.txt");
        let (empty_column, empty_rows, points) = parse_input(&input);
        assert_eq!(points.len(), 9);
        assert_eq!(empty_column, [2, 5, 8]);
        assert_eq!(empty_rows, [3, 7])
    }

    #[test]
    fn it_works() {
        let input = include_str!("./input_test.txt");
        let result = process(input);
        assert_eq!(result, "374");
    }
}
