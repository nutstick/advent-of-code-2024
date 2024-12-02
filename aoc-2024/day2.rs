fn is_safe(nums: &Vec<i32>) -> bool {
    let mut order = 0;
    for i in 1..nums.len() {
        let diff = nums[i] - nums[i - 1];
        match diff {
            1 | 2 | 3 => {
                if order == -1 {
                    return false;
                }
                order = 1;
            }
            -1 | -2 | -3 => {
                if order == 1 {
                    return false;
                }
                order = -1;
            }
            _ => {
                return false;
            }
        }
    }
    true
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut safe = 0;
    input.lines().for_each(|line| {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if is_safe(&nums) {
            safe += 1;
        }
    });

    safe
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let mut safe = 0;
    input.lines().for_each(|line| {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_safe(&nums) {
            safe += 1;
            return;
        }
        for i in 0..nums.len() {
            // remove i-th element from nums
            let mut modified = nums.clone();
            modified.remove(i);

            if is_safe(&modified) {
                safe += 1;
                return;
            }
        }
    });

    safe
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 2);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 4);
    }
}
