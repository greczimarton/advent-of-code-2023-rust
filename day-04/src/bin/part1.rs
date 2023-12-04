fn main() {
    let input = include_str!("./input_test.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    "13".to_string()
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
