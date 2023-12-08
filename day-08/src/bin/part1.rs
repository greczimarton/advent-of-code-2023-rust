use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut lines: Vec<&str> = input.lines().collect();
    lines.reverse();

    let instructions: Vec<char> = lines
        .pop()
        .expect("should have first instruction line")
        .chars()
        .collect();
    lines.pop();
    let map = lines
        .iter()
        .map(|map_line| MapNode::new(&map_line))
        .map(|node| (node.node_label, (node.left, node.right)))
        .collect::<HashMap<String, (String, String)>>();

    let mut current_node = "AAA";
    let mut step_count = 0;
    let mut i = 0;

    while current_node != "ZZZ" {
        step_count += 1;
        let current_direction = instructions[i];
        dbg!(current_direction);

        let temp = &map[current_node];
        if current_direction == 'L' {
            current_node = &temp.0;
        } else if current_direction == 'R' {
            current_node = &temp.1;
        } else {
            panic!("Invalid direction instruction {}", current_direction);
        }

        println!("moving to {}", current_node);

        if current_node == "ZZZ" {
            return step_count.to_string();
        }

        if i == instructions.len() - 1 {
            i = 0;
        } else {
            i += 1;
        }
    }
    panic!("couldn't get to ZZZ :(")
}

#[derive(Debug)]
struct MapNode {
    node_label: String,
    left: String,
    right: String,
}

impl MapNode {
    fn new(line: &str) -> Self {
        let binding = line
            .replace("=", "")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "");

        let data: Vec<&str> = binding.split_whitespace().collect();

        MapNode {
            node_label: data.iter().nth(0).expect("should have label").to_string(),
            left: data.iter().nth(1).expect("should have left").to_string(),
            right: data.iter().nth(2).expect("should have right").to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let input = include_str!("./input_1_test1.txt");
        let result = process(input);
        assert_eq!(result, "2");
    }

    #[test]
    fn it_works_2() {
        let input = include_str!("./input_1_test2.txt");
        let result = process(input);
        assert_eq!(result, "6");
    }
}
