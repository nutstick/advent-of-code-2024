use std::collections::HashSet;

use crate::{direction::Direction, vec2d::Vec2D};

const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

pub fn dfs(map: &Vec2D<char>, start: (usize, usize)) -> i32 {
    let mut stack = vec![(0, start)];
    let mut visited = HashSet::new();

    while let Some((value, (x, y))) = stack.pop() {
        for dir in DIRS {
            if let Some((nx, ny)) = dir.checked_add((x, y)) {
                if let Some(c) = map.get(nx, ny) {
                    if *c.to_string() == (value + 1).to_string() {
                        if *c == '9' {
                            visited.insert((nx, ny));
                        }
                        stack.push((value + 1, (nx, ny)));
                    }
                }
            }
        }
    }

    visited.len() as i32
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let mut total: i32 = 0;

    let map = Vec2D::new_chars(input);

    for i in 0..map.size.0 {
        for j in 0..map.size.1 {
            if let Some(c) = map.get(i, j) {
                if *c == '0' {
                    total += dfs(&map, (i, j));
                }
            }
        }
    }

    total
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> i32 {
    let mut total: i32 = 0;

    let mut queue = vec![HashSet::<(usize, usize)>::new(); 10];
    let map = Vec2D::new_chars(input);
    let mut rank = Vec2D::new_default(map.size, 0);

    for i in 0..map.size.0 {
        for j in 0..map.size.1 {
            if let Some(c) = map.get(i, j) {
                if *c == '0' {
                    queue[0].insert((i, j));
                    rank.set(i, j, 1);
                }
            }
        }
    }

    for i in 0..9 {
        for (x, y) in queue[i].clone() {
            for dir in DIRS {
                if let Some((nx, ny)) = dir.checked_add((x, y)) {
                    if let Some(c) = map.get(nx, ny) {
                        if *c.to_string() == (i + 1).to_string() {
                            queue.get_mut(i + 1).unwrap().insert((nx, ny));
                            if i == 8 {
                                total += rank.get(x, y).unwrap();
                            } else {
                                rank.set(
                                    nx,
                                    ny,
                                    rank.get(nx, ny).unwrap() + rank.get(x, y).unwrap(),
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 36);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 81);
    }
}
