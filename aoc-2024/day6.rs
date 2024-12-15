use std::collections::HashSet;

use crate::direction::Direction;
use crate::vec2d::Vec2D;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> i32 {
    let map = Vec2D::new_chars(input);
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut visited_pos: HashSet<(usize, usize)> = HashSet::new();

    let mut pos: (usize, usize) = (0, 0);
    let mut dir = Direction::Up;
    for i in 0..map.size.0 {
        for j in 0..map.size.1 {
            if let Some(c) = map.get(i, j) {
                if *c == '^' {
                    pos = (i, j);
                }
            }
        }
    }

    while pos.0 < map.size.0 && pos.1 < map.size.1 {
        if visited.contains(&(pos.0, pos.1, dir)) {
            break;
        }
        visited.insert((pos.0, pos.1, dir));
        visited_pos.insert((pos.0, pos.1));
        match dir {
            Direction::Up => {
                if let Some(c) = map.get(pos.0 - 1, pos.1) {
                    if *c != '#' {
                        pos.0 -= 1;
                    } else {
                        dir = Direction::Right;
                    }
                }
            }
            Direction::Down => {
                if let Some(c) = map.get(pos.0 + 1, pos.1) {
                    if *c != '#' {
                        pos.0 += 1;
                    } else {
                        dir = Direction::Left;
                    }
                }
            }
            Direction::Left => {
                if let Some(c) = map.get(pos.0, pos.1 - 1) {
                    if *c != '#' {
                        pos.1 -= 1;
                    } else {
                        dir = Direction::Up;
                    }
                }
            }
            Direction::Right => {
                if let Some(c) = map.get(pos.0, pos.1 + 1) {
                    if *c != '#' {
                        pos.1 += 1;
                    } else {
                        dir = Direction::Down;
                    }
                }
            }
            _ => {}
        }
    }

    visited_pos.len() as i32
}

fn is_loop(map: &Vec2D<char>, start: (usize, usize)) -> bool {
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();

    let mut pos: (usize, usize) = start.clone();
    let mut dir = Direction::Up;

    while pos.0 < map.size.0 && pos.1 < map.size.1 {
        if visited.contains(&(pos.0, pos.1, dir)) {
            return true;
        }
        visited.insert((pos.0, pos.1, dir));
        match dir {
            Direction::Up => {
                if pos.0 <= 0 {
                    return false;
                }
                if let Some(c) = map.get(pos.0 - 1, pos.1) {
                    if *c != '#' {
                        pos.0 -= 1;
                    } else {
                        dir = Direction::Right;
                    }
                } else {
                    return false;
                }
            }
            Direction::Down => {
                if let Some(c) = map.get(pos.0 + 1, pos.1) {
                    if *c != '#' {
                        pos.0 += 1;
                    } else {
                        dir = Direction::Left;
                    }
                } else {
                    return false;
                }
            }
            Direction::Left => {
                if pos.1 <= 0 {
                    return false;
                }
                if let Some(c) = map.get(pos.0, pos.1 - 1) {
                    if *c != '#' {
                        pos.1 -= 1;
                    } else {
                        dir = Direction::Up;
                    }
                } else {
                    return false;
                }
            }
            Direction::Right => {
                if let Some(c) = map.get(pos.0, pos.1 + 1) {
                    if *c != '#' {
                        pos.1 += 1;
                    } else {
                        dir = Direction::Down;
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }

    false
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i32 {
    let mut map = Vec2D::new_chars(input);

    let mut start: (usize, usize) = (0, 0);
    for i in 0..map.size.0 {
        for j in 0..map.size.1 {
            if let Some(c) = map.get(i, j) {
                if *c == '^' {
                    start = (i, j);
                }
            }
        }
    }

    let mut count = 0;
    for i in 0..map.size.0 {
        for j in 0..map.size.1 {
            let c = map.get(i, j).unwrap().clone();
            if c == '#' || c == '^' {
                continue;
            }
            map.set(i, j, '#');

            if is_loop(&map, start) {
                count += 1;
            }

            map.set(i, j, c);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 41);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 6);
    }
}
