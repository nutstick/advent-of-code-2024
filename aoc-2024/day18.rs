use std::collections::{HashSet, VecDeque};

use crate::{direction::Direction, vec2d::Vec2D};

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

struct Map {
    blocks: Vec<(usize, usize)>,
    first: (usize, usize),
    last: (usize, usize),
    size: usize,
}

impl Map {
    fn from_input(input: &str, size: usize) -> Self {
        let mut blocks = Vec::new();
        let first = (0, 0);
        let last = (size - 1, size - 1);

        input.lines().for_each(|line| {
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            blocks.push((x, y));
        });

        Self {
            blocks,
            first,
            last,
            size,
        }
    }

    fn shortest_path(&self, blocks: Vec<(usize, usize)>) -> Result<i32, String> {
        let mut grid = Vec2D::new_default((self.size, self.size), '.');
        blocks.iter().for_each(|(x, y)| {
            grid.set(*y, *x, '#');
        });

        let mut queue: VecDeque<((usize, usize), i32)> =
            VecDeque::from(vec![(self.first.clone(), 0)]);
        let mut visited = HashSet::new();
        while let Some(((x, y), cost)) = queue.pop_front() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            if self.last == (x, y) {
                return Ok(cost);
            }

            for dir in DIRECTIONS {
                if let Some((nx, ny)) = dir.checked_add((x, y)) {
                    if grid.get(nx, ny) == Some(&'.') {
                        queue.push_back(((nx, ny), cost + 1));
                    }
                }
            }
        }

        Err("No path found".to_string())
    }

    fn first_unreachable(&self) -> (usize, usize) {
        let mut st = 0;
        let mut ed = self.blocks.len();

        while st < ed {
            let mid = (st + ed) / 2;
            let blocks = self.blocks[..mid].to_vec();
            let result = self.shortest_path(blocks);

            if result.is_ok() {
                st = mid + 1;
            } else {
                ed = mid;
            }
        }

        self.blocks[ed - 1]
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> Result<i32, String> {
    let map = Map::from_input(input, 71);

    map.shortest_path(map.blocks[..1024].to_vec())
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> String {
    let map = Map::from_input(input, 71);

    let point = map.first_unreachable();
    format!("{},{}", point.0, point.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_sample_part_1() {
        let map = Map::from_input(SAMPLE_INPUT_1, 7);

        assert_eq!(22, map.shortest_path(map.blocks[..12].to_vec()).unwrap());
    }

    #[test]
    fn test_sample_part_2() {
        let map = Map::from_input(SAMPLE_INPUT_1, 7);

        assert_eq!((6, 1), map.first_unreachable());
    }
}
