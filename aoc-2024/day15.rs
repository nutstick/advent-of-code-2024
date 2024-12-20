use crate::{direction::Direction, vec2d::Vec2D};

fn move_stone(
    map: &mut Vec2D<char>,
    robot: (usize, usize),
    direction: Direction,
) -> (usize, usize) {
    let mut pos = robot.clone();
    let mut first_empty_space = robot.clone();
    loop {
        if let Some(next) = direction.checked_add(pos) {
            if let Some(c) = map.get(next.0, next.1) {
                match *c {
                    '.' => {
                        first_empty_space = next;
                        break;
                    }
                    '#' => {
                        break;
                    }
                    _ => {
                        pos = next;
                    }
                }
            }
        }
    }

    let mut previous = robot.clone();
    while robot != first_empty_space {
        if let Some(next) = direction.opposite().checked_add(first_empty_space) {
            if let Some(c) = map.get(next.0, next.1) {
                let current = map
                    .get(first_empty_space.0, first_empty_space.1)
                    .unwrap()
                    .clone();
                map.set(first_empty_space.0, first_empty_space.1, *c);
                map.set(next.0, next.1, current);
                previous = first_empty_space.clone();
                first_empty_space = next;
            }
        }
    }

    previous
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> i32 {
    let (map_input, commands_input) = input.split_once("\n\n").unwrap();
    let mut map = Vec2D::new_chars(map_input);
    let mut robot = map.find_first('@').unwrap();

    for cmd in commands_input.chars() {
        let direction = match cmd {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        };
        if let Some(direction) = direction {
            robot = move_stone(&mut map, robot, direction);
        }
        // println!("{}", map);
    }

    map.find_all('O')
        .iter()
        .map(|(x, y)| (*x as i32) * 100 + *y as i32)
        .sum()
}

struct Map {
    map: Vec2D<char>,
    robot: (usize, usize),
}

impl Map {
    pub fn from_input(input: &str) -> Self {
        let x = input.lines().count();
        let y = input.lines().next().unwrap().len() * 2;
        let mut map = Vec2D::<char>::new_default((x, y), ' ');

        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '@' => {
                        map.set(i, j * 2, '@');
                        map.set(i, j * 2 + 1, '.');
                    }
                    '#' => {
                        map.set(i, j * 2, '#');
                        map.set(i, j * 2 + 1, '#');
                    }
                    'O' => {
                        map.set(i, j * 2, '[');
                        map.set(i, j * 2 + 1, ']');
                    }
                    '.' => {
                        map.set(i, j * 2, '.');
                        map.set(i, j * 2 + 1, '.');
                    }
                    _ => {}
                }
            }
        }

        let robot = map.find_first('@').unwrap();

        Self { map, robot }
    }

    pub fn try_move(&mut self, position: (usize, usize), direction: Direction) -> bool {
        let current = self.map.get(position.0, position.1).unwrap();
        // println!("Try move: {:?} {:?} ({})", position, direction, current);
        if *current == '@' {
            if let Some(next) = direction.checked_add(position) {
                return match self.map.get(next.0, next.1) {
                    Some('.') => true,
                    Some('[') => self.try_move(next, direction),
                    Some(']') => self.try_move((next.0, next.1 - 1), direction),
                    Some('#') => false,
                    _ => panic!("Invalid state"),
                };
            }
            return false;
        }

        match direction {
            Direction::Up | Direction::Down => {
                match (
                    direction.checked_add(position),
                    direction.checked_add((position.0, position.1 + 1)),
                ) {
                    (Some(next_left), Some(next_right)) => {
                        let left_movable = match self.map.get(next_left.0, next_left.1) {
                            Some('.') => true,
                            Some('#') => false,
                            Some('[') => self.try_move(next_left, direction),
                            Some(']') => self.try_move((next_left.0, next_left.1 - 1), direction),
                            _ => panic!("Invalid character"),
                        };

                        let right_movable = match self.map.get(next_right.0, next_right.1) {
                            Some('.') => true,
                            Some('#') => false,
                            Some('[') => self.try_move(next_right, direction),
                            Some(']') => true,
                            _ => panic!("Invalid character"),
                        };

                        left_movable && right_movable
                    }
                    _ => false,
                }
            }
            Direction::Left => match direction.checked_add(position) {
                Some(next_left) => match self.map.get(next_left.0, next_left.1) {
                    Some('.') => true,
                    Some('#') => false,
                    Some('[') => panic!("Impossible state"),
                    Some(']') => self.try_move((next_left.0, next_left.1 - 1), direction),
                    _ => unimplemented!(),
                },
                _ => false,
            },
            Direction::Right => match direction.checked_add((position.0, position.1 + 1)) {
                Some(next_left) => match self.map.get(next_left.0, next_left.1) {
                    Some('.') => true,
                    Some('#') => false,
                    Some(']') => panic!("Impossible state"),
                    Some('[') => self.try_move(next_left, direction),
                    _ => unimplemented!(),
                },
                _ => false,
            },
            _ => panic!("Invalid direction"),
        }
    }

    pub fn commit_move(&mut self, position: (usize, usize), direction: Direction) {
        let current = self.map.get(position.0, position.1).unwrap();
        // println!(
        //     "Committing move: {:?} {:?} ({})",
        //     position, direction, current
        // );
        if *current == '@' {
            if let Some(next) = direction.checked_add(position) {
                match self.map.get(next.0, next.1) {
                    Some('.') => {}
                    Some('[') => self.commit_move(next, direction),
                    Some(']') => self.commit_move((next.0, next.1 - 1), direction),
                    _ => panic!("Invalid move"),
                }
                self.map.set(next.0, next.1, '@');
                self.map.set(position.0, position.1, '.');
                self.robot = next;
            }
            return;
        }

        match direction {
            Direction::Up | Direction::Down => match (
                direction.checked_add(position),
                direction.checked_add((position.0, position.1 + 1)),
            ) {
                (Some(next_left), Some(next_right)) => {
                    // Move block before it
                    match self.map.get(next_left.0, next_left.1) {
                        Some('.') => {}
                        Some('[') => {
                            self.commit_move(next_left, direction);
                        }
                        Some(']') => {
                            self.commit_move((next_left.0, next_left.1 - 1), direction);
                        }
                        _ => panic!("Invalid move"),
                    };

                    self.map.set(next_left.0, next_left.1, '[');
                    self.map.set(position.0, position.1, '.');

                    match self.map.get(next_right.0, next_right.1) {
                        Some('.') => {}
                        Some('[') => {
                            self.commit_move(next_right, direction);
                        }
                        Some(']') => {}
                        _ => panic!("Invalid move"),
                    };

                    self.map.set(next_right.0, next_right.1, ']');
                    self.map.set(position.0, position.1 + 1, '.');
                }
                _ => panic!("Invalid move"),
            },

            Direction::Left => match direction.checked_add(position) {
                Some(next_left) => {
                    match self.map.get(next_left.0, next_left.1) {
                        Some('.') => {}
                        Some(']') => {
                            self.commit_move((next_left.0, next_left.1 - 1), direction);
                        }
                        _ => panic!("Invalid move"),
                    }

                    self.map.set(next_left.0, next_left.1, '[');
                    self.map.set(position.0, position.1, ']');
                    self.map.set(position.0, position.1 + 1, '.');
                }
                _ => panic!("Invalid move"),
            },
            Direction::Right => match direction.checked_add((position.0, position.1 + 1)) {
                Some(next_left) => {
                    match self.map.get(next_left.0, next_left.1) {
                        Some('.') => {}
                        Some('[') => {
                            self.commit_move(next_left, direction);
                        }
                        _ => panic!("Invalid move"),
                    }

                    self.map.set(next_left.0, next_left.1, ']');
                    self.map.set(position.0, position.1 + 1, '[');
                    self.map.set(position.0, position.1, '.');
                }
                _ => panic!("Invalid move"),
            },
            _ => panic!("Invalid move"),
        }
    }
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> i32 {
    let (map_input, commands_input) = input.split_once("\n\n").unwrap();
    let mut map = Map::from_input(map_input);
    let commands = commands_input.trim_end().chars();

    // println!("{}", map.map);

    for cmd in commands {
        let direction = match cmd {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        };

        // println!("Trying to move: {:?}", direction);

        if let Some(direction) = direction {
            if map.try_move(map.robot, direction) {
                map.commit_move(map.robot, direction);
            }
        }

        // println!("{}", map.map);
        // wait for input
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
    }

    map.map
        .find_all('[')
        .iter()
        .map(|(x, y)| (*x as i32) * 100 + *y as i32)
        .sum()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    const SAMPLE_INPUT_2: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 10092);
    }

    #[test]
    fn test_sample_part_2_simple_test() {
        assert_eq!(part2(SAMPLE_INPUT_2), 618);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 9021);
    }
}
