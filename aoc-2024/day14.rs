use std::collections::HashSet;

use itertools::Itertools;

use crate::{direction::Direction, vec2d::Vec2D};

const MAX_TILES_X: i32 = 101;
const MAX_TILES_Y: i32 = 103;

#[derive(Debug)]
struct Robot {
    pub position: (i32, i32),
    velocity: (i32, i32),
}

fn parse_coordinate(part: &str) -> (i32, i32) {
    let coordinate_str = part.replace("p=", "").replace("v=", "");
    let mut pos = coordinate_str.split(',').map(|x| x.parse::<i32>().unwrap());
    (pos.next().unwrap(), pos.next().unwrap())
}

impl Robot {
    pub fn new(position: (i32, i32), velocity: (i32, i32)) -> Self {
        Self { position, velocity }
    }

    pub fn from_input(input: &str) -> Self {
        let (position_str, velocity_str) = input.split_once(" ").unwrap();

        Robot::new(
            parse_coordinate(position_str),
            parse_coordinate(velocity_str),
        )
    }

    fn next(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        if self.position.0 < 0 {
            self.position.0 += MAX_TILES_X;
        }
        if self.position.0 >= MAX_TILES_X {
            self.position.0 -= MAX_TILES_X;
        }
        if self.position.1 < 0 {
            self.position.1 += MAX_TILES_Y;
        }
        if self.position.1 >= MAX_TILES_Y {
            self.position.1 -= MAX_TILES_Y;
        }
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> i32 {
    let mut robots = input
        .lines()
        .map(|line| Robot::from_input(line))
        .collect_vec();

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.next();
        }
    }

    let mut count = vec![vec![0; 2]; 2];

    for robot in robots.iter_mut() {
        if robot.position.0 < MAX_TILES_X / 2 && robot.position.1 < MAX_TILES_Y / 2 {
            count[0][0] += 1;
        } else if robot.position.0 > MAX_TILES_X / 2 && robot.position.1 < MAX_TILES_Y / 2 {
            count[0][1] += 1;
        } else if robot.position.0 < MAX_TILES_X / 2 && robot.position.1 > MAX_TILES_Y / 2 {
            count[1][0] += 1;
        } else if robot.position.0 > MAX_TILES_X / 2 && robot.position.1 > MAX_TILES_Y / 2 {
            count[1][1] += 1;
        }
    }

    count[0][0] * count[0][1] * count[1][0] * count[1][1]
}

const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

fn count_connections(map: &Vec2D<char>) -> u32 {
    let mut count = 0;
    let mut visited = HashSet::new();

    for i in 0..MAX_TILES_Y as usize {
        for j in 0..MAX_TILES_X as usize {
            if *map.get(i, j).unwrap() == '#' && !visited.contains(&(i, j)) {
                count += 1;
                visited.insert((i, j));

                let mut stack = vec![(i, j)];
                while let Some((x, y)) = stack.pop() {
                    for dir in DIRS {
                        if let Some((nx, ny)) = dir.checked_add((x, y)) {
                            if let Some(c) = map.get(nx, ny) {
                                if *c == '#' && !visited.contains(&(nx, ny)) {
                                    visited.insert((nx, ny));
                                    stack.push((nx, ny));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
    let mut robots = input
        .lines()
        .map(|line| Robot::from_input(line))
        .collect_vec();

    for i in 0..1000000 {
        for robot in robots.iter_mut() {
            robot.next();
        }

        let mut map = Vec2D::new_default((MAX_TILES_Y as usize, MAX_TILES_X as usize), '.');
        for robot in robots.iter() {
            map.set(robot.position.0 as usize, robot.position.1 as usize, '#');
        }

        // Filter candidates, assuming to construct a christmas tree, there must
        // be a group with many connected cell
        let connections = count_connections(&map);
        if connections < 300 {
            println!("{}", map);
            println!("{} {}", i + 1, connections);

            return i + 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 12);
    }
}
