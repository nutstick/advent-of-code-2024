use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::{direction::Direction, vec2d::Vec2D};

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    position: (usize, usize),
    direction: Direction,
}

impl State {
    fn new(position: (usize, usize), direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: i32,
    current: State,
    previous: State,
}

impl Node {
    fn new(cost: i32, position: (usize, usize), direction: Direction) -> Self {
        Self {
            cost,
            current: State::new(position, direction),
            previous: State::new((0, 0), Direction::Up),
        }
    }

    fn with_previous(cost: i32, current: State, previous: State) -> Self {
        Self {
            cost,
            current,
            previous,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.current.position.cmp(&other.current.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> i32 {
    let map = Vec2D::new_chars(input);
    let start = map.find_first('S').unwrap();
    let end = map.find_first('E').unwrap();

    let mut visited = HashSet::<((usize, usize), Direction)>::new();

    let mut heap = BinaryHeap::new();

    heap.push(Node::new(0, start, Direction::Right));

    let mut min = i32::MAX;

    while let Some(Node {
        cost,
        current: State {
            position,
            direction,
        },
        previous: _,
    }) = heap.pop()
    {
        if position == end {
            min = min.min(cost);
            continue;
        }

        if visited.contains(&(position, direction)) {
            continue;
        }

        visited.insert((position, direction));

        let mut next_direction = direction.clone();
        for i in 0..4 {
            if let Some(next) = next_direction.checked_add(position) {
                match map.get(next.0, next.1) {
                    Some('.') | Some('E') => {
                        heap.push(Node::new(
                            cost + match i {
                                0 => 1,
                                1 => 1001,
                                2 => 2001,
                                3 => 1001,
                                _ => unreachable!(),
                            },
                            next,
                            next_direction.clone(),
                        ));
                    }
                    _ => {}
                }
            }
            next_direction = next_direction.clockwise();
        }
    }

    min
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> i32 {
    let map = Vec2D::new_chars(input);
    let start = map.find_first('S').unwrap();
    let end = map.find_first('E').unwrap();

    let mut min_cost = HashMap::<State, i32>::new();
    let mut backtrack = HashMap::<State, Vec<State>>::new();

    let mut heap = BinaryHeap::new();

    heap.push(Node::new(0, start, Direction::Right));

    while let Some(Node {
        cost,
        current,
        previous,
    }) = heap.pop()
    {
        let min_cost_at_position = min_cost.get(&current).unwrap_or(&i32::MAX);

        if *min_cost_at_position < cost {
            continue;
        } else if *min_cost_at_position == cost {
            backtrack.get_mut(&current).map(|v| v.push(previous));
            continue;
        } else if *min_cost_at_position == i32::MAX {
            min_cost.insert(current, cost);

            backtrack.insert(current, Vec::new());
            backtrack.get_mut(&current).map(|v| v.push(previous));
        } else {
            unreachable!()
        }

        if current.position == end {
            continue;
        }

        let mut next_direction = current.direction.clone();
        for i in 0..4 {
            if let Some(next) = next_direction.checked_add(current.position) {
                match map.get(next.0, next.1) {
                    Some('.') | Some('E') => {
                        heap.push(Node::with_previous(
                            cost + match i {
                                0 => 1,
                                1 => 1001,
                                2 => 2001,
                                3 => 1001,
                                _ => unreachable!(),
                            },
                            State::new(next, next_direction.clone()),
                            current,
                        ));
                    }
                    _ => {}
                }
            }
            next_direction = next_direction.clockwise();
        }
    }

    let mut stack = Vec::<State>::new();

    let min = DIRECTIONS
        .iter()
        .map(|d| min_cost.get(&State::new(end, *d)).unwrap_or(&i32::MAX))
        .min()
        .unwrap();

    for d in DIRECTIONS.iter() {
        match min_cost.get(&State::new(end, *d)) {
            Some(c) if c == min => {
                stack.push(State::new(end, *d));
            }
            _ => {}
        }
    }

    let mut visit = HashSet::<(usize, usize)>::new();

    while let Some(state) = stack.pop() {
        visit.insert(state.position);

        if state.position == start {
            continue;
        }

        for prev_state in backtrack.get(&state).unwrap() {
            stack.push(*prev_state);
        }
    }

    visit.len() as i32
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 7036);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 45);
    }
}
