use std::collections::HashMap;

use crate::vec2d::Vec2D;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i32 {
    let mut total = 0;
    let mut map = Vec2D::new_chars(input);
    let mut freq: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..map.size.0 {
        for j in 0..map.size.1 {
            if let Some(c) = map.get(i, j) {
                if c.is_alphanumeric() {
                    if !freq.contains_key(c) {
                        freq.insert(*c, vec![]);
                    }
                    freq.get_mut(c).unwrap().push((i, j));
                }
            }
        }
    }

    for (_, v) in freq.iter() {
        for i in 0..v.len() {
            for j in 0..v.len() {
                if i == j {
                    continue;
                }
                let (x0, y0) = v[i];
                let (x1, y1) = v[j];
                let antenna = (
                    x0 as i32 - x1 as i32 + x0 as i32,
                    y0 as i32 - y1 as i32 + y0 as i32,
                );
                if antenna.0 >= 0
                    && antenna.1 >= 0
                    && antenna.0 < map.size.0 as i32
                    && antenna.1 < map.size.1 as i32
                {
                    map.set(antenna.0 as usize, antenna.1 as usize, '#');
                }
            }
        }
    }

    for i in 0..map.size.0 {
        for j in 0..map.size.1 {
            if let Some(c) = map.get(i, j) {
                if *c == '#' {
                    total += 1;
                }
            }
        }
    }

    total
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i32 {
    let mut total = 0;
    let mut map = Vec2D::new_chars(input);
    let mut freq: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..map.size.0 {
        for j in 0..map.size.1 {
            if let Some(c) = map.get(i, j) {
                if c.is_alphanumeric() {
                    if !freq.contains_key(c) {
                        freq.insert(*c, vec![]);
                    }
                    freq.get_mut(c).unwrap().push((i, j));
                }
            }
        }
    }

    for (_, v) in freq.iter() {
        for i in 0..v.len() {
            for j in 0..v.len() {
                if i == j {
                    continue;
                }
                let (x0, y0) = v[i];
                let (x1, y1) = v[j];
                let step0 = x0 as i32 - x1 as i32;
                let step1 = y0 as i32 - y1 as i32;
                let mut antenna = (x0 as i32, y0 as i32);
                while antenna.0 >= 0
                    && antenna.1 >= 0
                    && antenna.0 < map.size.0 as i32
                    && antenna.1 < map.size.1 as i32
                {
                    map.set(antenna.0 as usize, antenna.1 as usize, '#');
                    antenna = (antenna.0 + step0, antenna.1 + step1);
                }
            }
        }
    }

    println!("{:?}", map);
    for i in 0..map.size.0 {
        for j in 0..map.size.1 {
            if let Some(c) = map.get(i, j) {
                if *c == '#' {
                    total += 1;
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 14);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 34);
    }
}
