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

    let start_nodes: Vec<String> = map
        .keys()
        .filter(|key| key.chars().last().expect("key should have last char") == 'A')
        .map(|start_key| start_key.clone())
        .collect();

    println!("Start");

    let mut max = 1;

    for node in start_nodes {
        let node_step_count: i64 = get_steps_for_node(node, &instructions, &map);
        dbg!(node_step_count);
        let lcm = lcm(node_step_count, max);
        if max < lcm {
            max = lcm;
        }
    }

    max.to_string()
}

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn get_steps_for_node(
    node: String,
    instructions: &Vec<char>,
    map: &HashMap<String, (String, String)>,
) -> i64 {
    let mut current_node = node.clone();
    let mut step_count = 0;
    let mut i = 0;

    while current_node.chars().last().expect("should have last char") != 'Z' {
        step_count += 1;
        let current_direction = instructions[i];

        let temp = &map[&current_node];
        dbg!(temp);
        if current_direction == 'L' {
            current_node = temp.0.to_string();
        } else if current_direction == 'R' {
            current_node = temp.1.to_string();
        } else {
            panic!("Invalid direction instruction {}", current_direction);
        }

        println!("moving to {}", current_node);

        if current_node.chars().last().expect("should have last char") == 'Z' {
            return step_count;
        }

        if i == instructions.len() - 1 {
            i = 0;
        } else {
            i += 1;
        }
    }

    panic!("couldn't find step_count for node: {node}");
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
    fn it_works() {
        let input = include_str!("./input_2_test.txt");
        let result = process(input);
        assert_eq!(result, "6");
    }
}
