use regex::{Match, Regex};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let regex_digits = Regex::new(r"(\d+)").unwrap();
    let regex_symbols = Regex::new(r"(\*)").unwrap();

    let mut digits: Vec<Digit> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        dbg!(&line);
        let mut temp_digits: Vec<Digit> = regex_digits
            .find_iter(line)
            .map(|a| Digit::new(a, i))
            .collect();

        digits.append(&mut temp_digits);

        let mut temp_symbols: Vec<Symbol> = regex_symbols
            .find_iter(line)
            .map(|a| Symbol::new(a, i))
            .collect();

        symbols.append(&mut temp_symbols);
    }

    let mut sum = 0;

    for symbol in symbols {
        let mut nearby_digits: Vec<&Digit> =
            digits.iter().filter(|d| d.row == symbol.row).collect();

        if symbol.row == 0 {
            //calc digits below
            let mut digits_below: Vec<&Digit> = digits.iter().filter(|d| d.row == 1).collect();
            nearby_digits.append(&mut digits_below)
        } else if symbol.row == i32::try_from(lines.len() - 1).expect("should parse usize to u32") {
            //calc digits only above
            let mut digits_above: Vec<&Digit> =
                digits.iter().filter(|d| d.row == symbol.row - 1).collect();
            nearby_digits.append(&mut digits_above);
        } else {
            //calc digits above and below
            let mut digits_above: Vec<&Digit> =
                digits.iter().filter(|d| d.row == symbol.row - 1).collect();

            let mut digits_below: Vec<&Digit> =
                digits.iter().filter(|d| d.row == symbol.row + 1).collect();

            nearby_digits.append(&mut digits_above);
            nearby_digits.append(&mut digits_below);
        }

        let mut adjacent_digits: Vec<&Digit> = Vec::new();

        for digit in nearby_digits {
            if Digit::is_adjacent(&digit, &symbol) {
                println!("Digit: {:?} good", digit);
                adjacent_digits.push(digit);
            } else {
                println!("Digit: {:?} bad", digit);
            }
        }

        if adjacent_digits.len() != 2 {
            continue;
        }

        sum += adjacent_digits[0].num * adjacent_digits[1].num;
    }

    sum.to_string()
}

#[derive(Debug)]
struct Digit {
    num: i32,
    row: i32,
    start_column: i32,
    end_column: i32,
}

impl Digit {
    pub fn new(regex_match: Match, line_index: usize) -> Self {
        Digit {
            num: regex_match
                .as_str()
                .parse::<i32>()
                .expect("should parse digit to u32"),
            row: i32::try_from(line_index).expect("should parse usize to u32"),
            start_column: i32::try_from(regex_match.start()).expect("should parse usize to u32"),
            end_column: i32::try_from(regex_match.end() - 1).expect("should parse usize to u32"),
        }
    }
}

#[derive(Debug)]
struct Symbol {
    row: i32,
    column: i32,
}

impl Symbol {
    pub fn new(regex_match: Match, line_index: usize) -> Self {
        Symbol {
            row: i32::try_from(line_index).expect("should parse usize to u32"),
            column: i32::try_from(regex_match.start()).expect("should parse usize to u32"),
        }
    }
}

impl Digit {
    fn is_adjacent(&self, symbol: &Symbol) -> bool {
        //isLeft
        if symbol.column - 1 == self.end_column {
            return true;
        }
        //isRight
        if symbol.column + 1 == self.start_column {
            return true;
        }
        //isTopLeft
        if symbol.row - 1 == self.row && symbol.column - 1 == self.end_column {
            return true;
        }
        //isBottomLeft
        if symbol.row + 1 == self.row && symbol.column - 1 == self.end_column {
            return true;
        }
        //isTopRIght
        if symbol.row - 1 == self.row && symbol.column + 1 == self.start_column {
            return true;
        }
        //isBottomRight
        if symbol.row + 1 == self.row && symbol.column + 1 == self.start_column {
            return true;
        }
        //isTop
        if symbol.row - 1 == self.row
            && (self.start_column - 1..self.end_column + 1).contains(&symbol.column)
        {
            return true;
        }
        //isBottom
        if symbol.row + 1 == self.row
            && ((self.start_column - 1)..(self.end_column + 1)).contains(&symbol.column)
        {
            return true;
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //     #[test]
    //     fn left_works() {
    //         let result = process("617*.....");
    //         let result1 = process(".....617*");
    //         assert_eq!(result, "617");
    //         assert_eq!(result1, "617");
    //     }

    //     #[test]
    //     fn right_works() {
    //         let result = process("......*617");
    //         let result1 = process("*617");
    //         assert_eq!(result, "617");
    //         assert_eq!(result1, "617");
    //     }

    //     #[test]
    //     fn above_works() {
    //         let result = process(
    //             "
    // 6.....
    // *.....",
    //         );
    //         //         let result1 = process(
    //         //             "
    //         // 61....*
    //         // *.....",
    //         //         );
    //         //         let result2 = process(
    //         //             "
    //         // .62....
    //         // .*.....",
    //         //         );
    //         //         let result3 = process(
    //         //             "
    //         // 62.....
    //         // .*.....",
    //         //         );
    //         //         let result4 = process(
    //         //             "
    //         // 612.....
    //         // .*.....",
    //         //         );
    //         //         let result5 = process(
    //         //             "
    //         // 6132...
    //         // ..*....",
    //         //         );
    //         assert_eq!(result, "6");
    //         // assert_eq!(result1, "61");
    //         // assert_eq!(result2, "62");
    //         // assert_eq!(result3, "62");
    //         // assert_eq!(result4, "612");
    //         // assert_eq!(result5, "6132");
    //     }

    #[test]
    fn it_works() {
        let result = process(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "467835");
    }
}
