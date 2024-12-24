use std::collections::VecDeque;

use crate::{direction::Direction, vec2d::Vec2D};

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

struct Map {
    grid: Vec2D<char>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn from_input(input: &str) -> Self {
        let grid = Vec2D::new_chars(input);
        let start = grid.find_first('S').unwrap();
        let end = grid.find_first('E').unwrap();

        Self { grid, start, end }
    }

    fn shortest_path(&self, start: (usize, usize)) -> Vec2D<i32> {
        let mut shortest_path = Vec2D::new_default(self.grid.size, -1);

        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        while let Some(((x, y), cost)) = queue.pop_front() {
            if *shortest_path.get(x, y).unwrap() != -1 {
                continue;
            }

            shortest_path.set(x, y, cost);
            for dir in DIRECTIONS {
                if let Some((nx, ny)) = dir.checked_add((x, y)) {
                    match self.grid.get(nx, ny) {
                        Some('.') | Some('E') | Some('S') => {
                            queue.push_back(((nx, ny), cost + 1));
                        }
                        _ => {}
                    }
                }
            }
        }

        shortest_path
    }

    fn count_threshold(&self, threshold: i32, time_limit: isize, exact: bool) -> i32 {
        let shortest_path_from_start = self.shortest_path(self.start);
        let shortest_path_from_end = self.shortest_path(self.end);

        let shortest_path = *shortest_path_from_start
            .get(self.end.0, self.end.1)
            .unwrap();

        let mut count = 0;
        for sx in 0..self.grid.size.0 {
            for sy in 0..self.grid.size.1 {
                for ex in 0..self.grid.size.0 {
                    for ey in 0..self.grid.size.1 {
                        let len_x: isize = (sx as isize) - (ex as isize);
                        let len_y = (sy as isize) - (ey as isize);
                        let len = len_x.abs() + len_y.abs();
                        if sx == ex && sy == ey {
                            continue;
                        }
                        if exact && len != time_limit {
                            continue;
                        }
                        if !exact && len > time_limit {
                            continue;
                        }

                        match (
                            shortest_path_from_start.get(sx, sy),
                            shortest_path_from_end.get(ex, ey),
                        ) {
                            (Some(from_start), Some(from_end))
                                if *from_start != -1 && *from_end != -1 =>
                            {
                                if shortest_path - from_start - from_end - (len as i32) >= threshold
                                {
                                    count += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        count
    }
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> i32 {
    let map = Map::from_input(input);

    map.count_threshold(100, 2, true)
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> i32 {
    let map = Map::from_input(input);

    map.count_threshold(100, 20, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_sample_part_1() {
        let map = Map::from_input(SAMPLE_INPUT_1);

        assert_eq!(1, map.count_threshold(60, 2, true));
        assert_eq!(16, map.count_threshold(6, 2, true));
    }

    #[test]
    fn test_sample_part_2() {
        let map = Map::from_input(SAMPLE_INPUT_1);

        assert_eq!(285, map.count_threshold(50, 20, false));
    }
}
