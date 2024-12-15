use std::collections::{HashMap, HashSet};

use crate::{direction::Direction, vec2d::Vec2D};

const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[aoc(day12, part1)]
pub fn part1(input: &str) -> i32 {
    let mut total = 0;
    let map = Vec2D::new_chars(input);
    let mut visited = HashSet::<(usize, usize)>::new();

    for i in 0..map.size.0 {
        for j in 0..map.size.1 {
            if visited.contains(&(i, j)) {
                continue;
            }

            let mut count = 1;
            let mut region = 0;
            let mut stack = vec![(i, j)];
            let current = map.get(i, j).unwrap();

            visited.insert((i, j));

            while let Some((x, y)) = stack.pop() {
                for dir in DIRS {
                    if let Some((nx, ny)) = dir.checked_add((x, y)) {
                        if let Some(c) = map.get(nx, ny) {
                            if *c == *current {
                                if !visited.contains(&(nx, ny)) {
                                    visited.insert((nx, ny));
                                    count += 1;

                                    stack.push((nx, ny));
                                }
                            } else {
                                region += 1;
                            }
                        } else {
                            region += 1;
                        }
                    } else {
                        region += 1;
                    }
                }
            }

            total += count * region;
        }
    }

    total
}

struct ConnectedPoint {
    map: HashMap<(usize, usize), HashSet<Direction>>,
}

impl ConnectedPoint {
    pub fn new() -> Self {
        Self {
            map: HashMap::<(usize, usize), HashSet<Direction>>::new(),
        }
    }

    fn mark_point(&mut self, x: usize, y: usize, direction: Direction) {
        let dirs = self.map.entry((x, y)).or_insert(HashSet::new());
        dirs.insert(direction);
    }

    pub fn mark(&mut self, x: usize, y: usize) {
        self.mark_point(x, y, Direction::DownRight);
        self.mark_point(x + 1, y, Direction::DownLeft);
        self.mark_point(x, y + 1, Direction::UpRight);
        self.mark_point(x + 1, y + 1, Direction::UpLeft);
    }

    pub fn count_corners(&self) -> i32 {
        let mut corner = 0;

        for (_, dirs) in &self.map {
            if dirs.len() % 2 == 1 {
                corner += 1;
            } else if dirs.len() == 2 {
                if dirs.contains(&Direction::DownRight) && dirs.contains(&Direction::UpLeft) {
                    corner += 2;
                } else if dirs.contains(&Direction::DownLeft) && dirs.contains(&Direction::UpRight)
                {
                    corner += 2;
                }
            }
        }

        corner
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> i32 {
    let mut total = 0;
    let map = Vec2D::new_chars(input);
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut connected_point = ConnectedPoint::new();

    for i in 0..map.size.0 {
        for j in 0..map.size.1 {
            if visited.contains(&(i, j)) {
                continue;
            }

            let mut count = 0;
            let mut stack = vec![(i, j)];
            let current = map.get(i, j).unwrap();

            visited.insert((i, j));
            connected_point.clear();

            while let Some((x, y)) = stack.pop() {
                count += 1;
                connected_point.mark(x, y);

                for dir in DIRS {
                    if let Some((nx, ny)) = dir.checked_add((x, y)) {
                        // Already visited
                        if visited.contains(&(nx, ny)) {
                            continue;
                        }

                        if let Some(c) = map.get(nx, ny) {
                            if *c == *current {
                                visited.insert((nx, ny));
                                stack.push((nx, ny));
                            }
                        }
                    }
                }
            }

            total += count * connected_point.count_corners();
        }
    }

    total
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    const SAMPLE_INPUT_2: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    const SAMPLE_INPUT_3: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 1930);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 1206);
    }

    #[test]
    fn test_sample_part_2_test_2() {
        assert_eq!(part2(SAMPLE_INPUT_2), 236);
    }

    #[test]
    fn test_sample_part_2_test_3() {
        assert_eq!(part2(SAMPLE_INPUT_3), 368);
    }
}
