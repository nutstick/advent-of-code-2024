use std::collections::{hash_map, hash_set};

use itertools::Itertools;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    let mut total = 0;
    let (edges, books) = input.split_once("\n\n").unwrap();

    let mut before_map: hash_map::HashMap<&str, Vec<&str>> = hash_map::HashMap::new();
    edges.lines().for_each(|l| {
        let (before, after) = l.split_once("|").unwrap();
        if !before_map.contains_key(&before) {
            before_map.insert(before, vec![]);
        }
        before_map.get_mut(&before).unwrap().push(after);
    });

    books.lines().for_each(|l| {
        let order = l.split(",").collect_vec();
        let mut visited = hash_set::HashSet::new();

        let mut success = true;
        for x in order.clone() {
            if let Some(before) = before_map.get(&x) {
                if before.iter().any(|&y| visited.contains(y)) {
                    success = false;
                    break;
                }
            }
            visited.insert(x);
        }

        if success {
            total += order.get(order.len() / 2).unwrap().parse::<i32>().unwrap();
        }
    });

    total
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    let mut total = 0;
    let (edges, books) = input.split_once("\n\n").unwrap();

    let mut before_map: hash_map::HashMap<&str, Vec<&str>> = hash_map::HashMap::new();
    edges.lines().for_each(|l| {
        let (before, after) = l.split_once("|").unwrap();
        if !before_map.contains_key(&before) {
            before_map.insert(before, vec![]);
        }
        before_map.get_mut(&before).unwrap().push(after);
    });

    books.lines().for_each(|l| {
        let order = l.split(",").collect_vec();
        let mut visited = hash_set::HashSet::new();

        let mut success = true;
        for x in order.clone() {
            if let Some(before) = before_map.get(&x) {
                // println!("{:?} {:?}", x, before);
                if before.iter().any(|&y| visited.contains(y)) {
                    success = false;
                    break;
                }
            }
            visited.insert(x);
        }

        if !success {
            let mut new_order = order.clone();
            new_order.sort_by(|a, b| {
                (before_map.contains_key(b) && before_map[b].contains(a)).cmp(&true)
            });
            total += new_order
                .get(new_order.len() / 2)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            // println!("total: {}", total);
        }
        // println!("---");
    });

    total
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 143);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 123);
    }
}
