fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let seeds: Vec<u64> = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|num| {
            num.parse::<u64>()
                .expect("should parse seed numbers to u32")
        })
        .collect();

    let maps = get_maps(lines);
    let mut min: Option<u64> = None;

    for seed in seeds {
        let location_value = map_seed_to_location(seed, &maps);
        match min {
            Some(current_min) => {
                if location_value < current_min {
                    min = Some(location_value)
                }
            }
            None => min = Some(location_value),
        }
    }

    min.expect("Should have minimum value").to_string()
}

fn map_seed_to_location(seed: u64, groups: &Vec<AlmanacGroup>) -> u64 {
    let mut current_val = seed;
    for group in groups {
        print!("{} {} {}", group.start_range, group.end_range, current_val);
        if (group.start_range..group.end_range).contains(&current_val) {
            current_val = map_source_in_group(current_val, group);
        } else {
            print!(" SHORTCUT");
        }
        println!(" {}", current_val);
    }
    println!("seed: {seed} location: {current_val}");

    current_val
}

fn map_source_in_group(source: u64, group: &AlmanacGroup) -> u64 {
    for map in group.maps.iter() {
        if map.is_source_in_range(source) {
            let delta = source - map.source_start;
            return map.destination_start + delta;
        }
    }

    source
}

fn get_maps(lines: Vec<&str>) -> Vec<AlmanacGroup> {
    let mut groups = vec![];
    let mut current_group = vec![];

    for line in lines.into_iter().skip(2) {
        if line.contains("map") {
            continue;
        }
        if !line.is_empty() {
            current_group.push(AlmanacMap::new(line));
            continue;
        }

        if !current_group.is_empty() {
            let almanac_group = AlmanacGroup::new(current_group);
            groups.push(almanac_group);
            current_group = vec![];
        }
    }

    if !current_group.is_empty() {
        let almanac_group = AlmanacGroup::new(current_group);
        groups.push(almanac_group);
    }
    groups
}

#[derive(Debug)]
struct AlmanacGroup {
    maps: Vec<AlmanacMap>,
    start_range: u64,
    end_range: u64,
}

impl AlmanacGroup {
    pub fn new(group: Vec<AlmanacMap>) -> Self {
        let min = group
            .iter()
            .min_by(|map1, map2| map1.source_start.cmp(&map2.source_start))
            .map(|map| map.source_start)
            .expect("should have min");

        let max = group
            .iter()
            .max_by(|map1, map2| {
                (map1.source_start + map1.range).cmp(&(map2.source_start + map2.range))
            })
            .map(|map| map.source_start + map.range)
            .expect("should have max");

        AlmanacGroup {
            maps: group,
            start_range: min,
            end_range: max,
        }
    }
}

#[derive(Debug)]
struct AlmanacMap {
    destination_start: u64,
    source_start: u64,
    range: u64,
}
// impl std::fmt::Display for AlmanacMap {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "(asd)")
//     }
// }
impl AlmanacMap {
    pub fn new(line: &str) -> Self {
        let nums: Vec<u64> = line
            .split(' ')
            .map(|num| num.parse::<u64>().expect("should map vals to u32"))
            .collect();

        AlmanacMap {
            destination_start: nums[0],
            source_start: nums[1],
            range: nums[2],
        }
    }

    fn is_source_in_range(&self, source: u64) -> bool {
        (self.source_start..(self.source_start + self.range)).contains(&source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input_test.txt");
        let result = process(input);
        assert_eq!(result, "35");
    }
}
