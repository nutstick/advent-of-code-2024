use std::collections::HashMap;

fn dp(cache: &mut HashMap<(u64, usize), u64>, value: u64, blink: usize) -> u64 {
    if blink == 0 {
        return 1;
    }
    if let Some(count) = cache.get(&(value, blink)) {
        return *count;
    }

    if value == 0 {
        let count = dp(cache, 1, blink - 1);
        cache.insert((value, blink), count);
        return count;
    } else if value.to_string().len() % 2 == 0 {
        let str = value.to_string();
        let (a, b) = str.split_at(str.len() / 2);
        let count = dp(cache, a.parse::<u64>().unwrap(), blink - 1)
            + dp(cache, b.parse::<u64>().unwrap(), blink - 1);
        cache.insert((value, blink), count);
        return count;
    } else {
        let count = dp(cache, value * 2024, blink - 1);
        cache.insert((value, blink), count);
        return count;
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    let mut total = 0;
    let mut cache = HashMap::new();
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for j in 0..stones.len() {
        total += dp(&mut cache, stones[j], 25);
    }

    total
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    let mut total = 0;
    let mut cache = HashMap::new();
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for j in 0..stones.len() {
        total += dp(&mut cache, stones[j], 75);
    }

    total
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
125 17";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 55312);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 65601038650482);
    }
}
