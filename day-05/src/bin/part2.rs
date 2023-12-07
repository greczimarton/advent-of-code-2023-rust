use std::ops::Range;
use std::{cmp, vec};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let seed_ranges: Vec<i64> = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|num| {
            num.parse::<i64>()
                .expect("should parse seed numbers to i64")
        })
        .collect();

    let seed_ranges_chunked: Vec<std::ops::Range<i64>> = seed_ranges
        .chunks(2)
        .map(|chunk| std::ops::Range {
            start: chunk[0],
            end: chunk[0] + chunk[1] - 1,
        })
        .collect();

    let groups: Vec<Vec<AlmanacMap>> = get_maps(lines);

    let mut current_intervals = seed_ranges_chunked.clone();

    for group in groups {
        let mut new_intervals: Vec<Range<i64>> = vec![];

        // println!("{i}. conversion: {:?}", current_intervals);

        for interval in current_intervals {
            // println!("Checking {:?}", interval);

            let mut mapped_interval_parts: Vec<Range<i64>> = vec![];
            let mut same_interval_parts: Vec<Range<i64>> = vec![];

            let mut retry_parts: Vec<Range<i64>> = vec![];
            retry_parts.push(interval);

            for i in 0..group.len() {
                let pop = retry_parts.pop();

                let Some(temp_interval) = pop else {
                    continue;
                };

                let (left, mid, right) = group[i].map_interval(temp_interval.clone());

                if let Some(left_int) = left {
                    if !left_int.is_empty() {
                        retry_parts.push(left_int);
                    }
                }

                if let Some(mid_int) = mid {
                    if !mid_int.is_empty() {
                        mapped_interval_parts.push(mid_int)
                    }
                }

                if let Some(right_int) = right {
                    if !right_int.is_empty() {
                        retry_parts.push(right_int)
                    }
                }

                if i == group.len() - 1 {
                    // println!("last");
                    same_interval_parts.append(&mut retry_parts);
                    retry_parts = vec![]
                }
            }

            new_intervals.append(&mut mapped_interval_parts);
            new_intervals.append(&mut same_interval_parts);
        }

        current_intervals = vec![];
        current_intervals.append(&mut new_intervals);
    }

    let min = current_intervals
        .iter()
        .map(|interval| interval.start)
        .min()
        .expect("should have min value");

    return min.to_string();
}

fn get_maps(lines: Vec<&str>) -> Vec<Vec<AlmanacMap>> {
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
            groups.push(current_group);
            current_group = vec![];
        }
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
}

#[derive(Debug)]
struct AlmanacMap {
    destination_start: i64,
    source_start: i64,
    range: i64,
}

impl AlmanacMap {
    pub fn new(line: &str) -> Self {
        let nums: Vec<i64> = line
            .split(' ')
            .map(|num| num.parse::<i64>().expect("should map vals to u32"))
            .collect();

        AlmanacMap {
            destination_start: nums[0],
            source_start: nums[1],
            range: nums[2],
        }
    }

    fn map_interval(
        &self,
        interval: Range<i64>,
    ) -> (Option<Range<i64>>, Option<Range<i64>>, Option<Range<i64>>) {
        // dbg!(&interval);
        // dbg!(Range {
        //     start: self.source_start,
        //     end: self.source_start + self.range
        // });
        //dont overlap
        if self.source_start + self.range < interval.start {
            // println!("src is left to interval");
            return (None, None, Some(interval));
        }
        if interval.end < self.source_start {
            // println!("src is right to interval");
            return (Some(interval), None, None);
        }

        // println!("src overlaps interval");

        let mut mid = Range {
            start: cmp::max(self.source_start, interval.start),
            end: cmp::min(self.source_start + self.range, interval.end),
        };

        let left = Range {
            start: interval.start,
            end: mid.start,
        };
        let right = Range {
            start: mid.end,
            end: interval.end,
        };

        let offset = self.destination_start - self.source_start;
        mid.start = mid.start + offset;
        mid.end = mid.end + offset;

        // dbg!(&left);
        // dbg!(&right);
        // println!("mid mapped {:?}", mid);

        return (Some(left), Some(mid), Some(right));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input_test.txt");
        let result = process(input);
        assert_eq!(result, "46");
    }
}
