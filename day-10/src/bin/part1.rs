use std::char;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let map = parse_input(&input);

    let start = map
        .iter()
        .flat_map(|row| row.iter())
        .find(|tile| tile.pipe == Pipe::Start)
        .expect("should have start");

    let (mut path1, mut path1_dir, mut path2, mut path2_dir) =
        get_first_two_connecting_pipes(start, &map);
    let mut current_step = 1;

    loop {
        current_step += 1;
        dbg!(&current_step);
        dbg!(&path1);
        dbg!(&path2);

        let (path1_next, path1_next_dir) = path1.get_next_step(&path1_dir, &map);
        let (path2_next, path2_next_dir) = path2.get_next_step(&path2_dir, &map);

        if path1_next == path2_next {
            return current_step.to_string();
        } else {
            path1 = path1_next;
            path2 = path2_next;
            path1_dir = path1_next_dir.flip();
            path2_dir = path2_next_dir.flip();
        }
    }
}

fn get_first_two_connecting_pipes(
    start: &Tile,
    map: &Vec<Vec<Tile>>,
) -> (Tile, Direction, Tile, Direction) {
    let mut path1: Option<Tile> = None;
    let mut path1_dir: Option<Direction> = None;
    let mut path2: Option<Tile> = None;
    let mut path2_dir: Option<Direction> = None;

    for direction in [
        Direction::East,
        Direction::North,
        Direction::South,
        Direction::West,
    ] {
        if path1.is_some() && path2.is_some() {
            break;
        }

        let next = start.get_next(&direction, &map);

        if next.is_none() {
            continue;
        }

        if path1.is_none() {
            path1 = Some(next.expect("should have next val").clone());
            path1_dir = Some(direction);
        } else {
            path2 = Some(next.expect("should have next val").clone());
            path2_dir = Some(direction);
        }
    }

    if path1.is_none() || path2.is_none() {
        panic!("couldn't find both path1 and path 2");
    }

    (
        path1.expect("should have path1"),
        path1_dir.expect("should have path1dir").flip(),
        path2.expect("should have path2"),
        path2_dir.expect("should have path2dir").flip(),
    )
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    let temp: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut map: Vec<Vec<Tile>> = vec![];

    for i in 0..temp.len() {
        map.push(vec![]);
        for j in 0..temp[i].len() {
            map[i].push(Tile::new(i, j, temp[i][j]))
        }
    }

    map
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Tile {
    row: usize,
    column: usize,
    pipe: Pipe,
}

impl Tile {
    fn new(row: usize, column: usize, character: char) -> Tile {
        Tile {
            row: row,
            column: column,
            pipe: Pipe::new(character),
        }
    }

    fn get_next_step(&self, prev_dir: &Direction, map: &Vec<Vec<Tile>>) -> (Tile, Direction) {
        let mut possible_directions = self.pipe.possible_directions();
        possible_directions.retain(|dir| dir != prev_dir);

        if possible_directions.len() > 1 {
            panic!("too many possible directions, shouldn't be possible")
        }

        let dir = possible_directions
            .iter()
            .nth(0)
            .expect("should have only direction");

        let Some(next) = self.get_next(dir, &map) else {
            panic!("couldn't go to the only direction, shouldn't be possible")
        };

        (next, dir.clone())
    }

    fn get_next(&self, direction: &Direction, map: &Vec<Vec<Tile>>) -> Option<Tile> {
        let other: &Tile = match direction {
            Direction::South => {
                if self.row + 1 >= map.len() {
                    return None;
                }

                map[self.row + 1]
                    .iter()
                    .nth(self.column)
                    .expect("should have val")
            }
            Direction::West => {
                if self.column == 0 {
                    return None;
                }

                map[self.row]
                    .iter()
                    .nth(self.column - 1)
                    .expect("should have val")
            }
            Direction::North => {
                if self.row == 0 {
                    return None;
                }

                map[self.row - 1]
                    .iter()
                    .nth(self.column)
                    .expect("should have val")
            }
            Direction::East => {
                if self.column + 1 >= map.len() {
                    return None;
                }
                map[self.row]
                    .iter()
                    .nth(self.column + 1)
                    .expect("should have val")
            }
        };

        if Tile::can_go(other, direction) {
            Some(other.clone())
        } else {
            None
        }
    }

    fn can_go(other: &Tile, direction: &Direction) -> bool {
        match direction {
            Direction::South => {
                [Pipe::Vertical, Pipe::NorthEast, Pipe::NorthWest].contains(&other.pipe)
            }
            Direction::West => {
                [Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast].contains(&other.pipe)
            }
            Direction::North => {
                [Pipe::Vertical, Pipe::SouthEast, Pipe::SouthWest].contains(&other.pipe)
            }
            Direction::East => {
                [Pipe::Horizontal, Pipe::SouthWest, Pipe::NorthWest].contains(&other.pipe)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Pipe {
    fn new(character: char) -> Pipe {
        match character {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("invalid input in map {character}"),
        }
    }

    fn possible_directions(&self) -> Vec<Direction> {
        match self {
            Pipe::Vertical => vec![Direction::North, Direction::South],
            Pipe::Horizontal => vec![Direction::West, Direction::East],
            Pipe::NorthEast => vec![Direction::North, Direction::East],
            Pipe::NorthWest => vec![Direction::North, Direction::West],
            Pipe::SouthWest => vec![Direction::South, Direction::West],
            Pipe::SouthEast => vec![Direction::South, Direction::East],
            Pipe::Ground => vec![],
            Pipe::Start => vec![
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    South,
    West,
    North,
    East,
}

impl Direction {
    fn flip(&self) -> Direction {
        match self {
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            Direction::East => Direction::West,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = include_str!("./input_test1_simple.txt");
        let map = parse_input(input);
        assert_eq!(map[1][1].pipe, Pipe::Start);
    }

    #[test]
    fn it_works_simple() {
        let input = include_str!("./input_test1_simple.txt");
        let result = process(input);
        assert_eq!(result, "4");
    }

    #[test]
    fn it_works_complex() {
        let input = include_str!("./input_test1_complex.txt");
        let result = process(input);
        assert_eq!(result, "8");
    }
}
