use std::{borrow::BorrowMut, collections::HashSet};

use itertools::Itertools;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    let mut total: u64 = 0;

    input.lines().for_each(|l| {
        let (ans_str, nums_str) = l.split_once(": ").unwrap();
        let ans = ans_str.parse::<u64>().unwrap();
        let nums = nums_str
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect_vec();
        let mut possible_ans: Vec<Vec<u64>> = vec![vec![0], vec![]];
        nums.iter().enumerate().for_each(|(i, num)| {
            let current = possible_ans.get(i % 2).unwrap().clone();
            let next = possible_ans.get_mut((i + 1) % 2).unwrap();

            next.clear();
            current.iter().for_each(|x| {
                next.push(x + num);
                next.push(x * num);
            });
        });
        if possible_ans.get(nums.len() % 2).unwrap().contains(&ans) {
            total += ans;
        }
    });

    total
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    let mut total: u64 = 0;

    input.lines().for_each(|l| {
        let (ans_str, nums_str) = l.split_once(": ").unwrap();
        let ans = ans_str.parse::<u64>().unwrap();
        let nums = nums_str
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect_vec();
        let mut possible_ans: [HashSet<u64>; 2] = [HashSet::new(), HashSet::new()];
        nums.iter().enumerate().for_each(|(i, num)| {
            if i == 0 {
                possible_ans[1].borrow_mut().insert(*num);
                return;
            }
            let current = possible_ans[i % 2].clone();
            let next = possible_ans[(i + 1) % 2].borrow_mut();

            next.clear();
            current.iter().for_each(|x| {
                next.insert(x + num);
                next.insert(x * num);
                next.insert(x * (10 as u64).pow(num.to_string().len() as u32) + num);
            });
        });
        if possible_ans[nums.len() % 2].iter().any(|x| *x == ans) {
            total += ans;
        }
    });

    total
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 3749);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 11387);
    }
}
