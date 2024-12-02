use std::collections::hash_map;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut total = 0;

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    // Remove first line
    input.lines().for_each(|l| match l.split_once("   ") {
        Some((word1, word2)) => {
            list1.push(word1.parse().unwrap());
            list2.push(word2.parse().unwrap());
        }
        None => {
            panic!("Invalid input: {}", l);
        }
    });

    list1.sort();
    list2.sort();

    for i in 0..list1.len() {
        total += (list1[i] - list2[i]).abs();
    }

    return total;
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut total = 0;

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    // Remove first line
    input.lines().for_each(|l| match l.split_once("   ") {
        Some((word1, word2)) => {
            list1.push(word1.parse().unwrap());
            list2.push(word2.parse().unwrap());
        }
        None => {
            panic!("Invalid input: {}", l);
        }
    });

    let mut map: hash_map::HashMap<i32, i32> = hash_map::HashMap::new();
    list2.iter().for_each(|&x| {
        if map.contains_key(&x) {
            map.insert(x, map.get(&x).unwrap() + 1);
        } else {
            map.insert(x, 1);
        }
    });
    list1.iter().for_each(|&x| {
        if map.contains_key(&x) {
            total += map.get(&x).unwrap() * x;
        }
    });

    total
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 11);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 31);
    }
}
