use std::cmp::min;

fn sum_range(start: u64, size: u64) -> u64 {
    if start == 0 {
        return (size - 1) * size / 2;
    }
    return (start + size - 1) * (start + size) / 2 - start * (start - 1) / 2;
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
    let mut total: u64 = 0;
    let mut nums = input
        .chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut digit = 0;
    let mut last_id = nums.len() / 2;

    let mut i: usize = 0;
    while i <= last_id * 2 {
        if i % 2 == 0 {
            let id = i as u64 / 2;
            let sum = sum_range(digit, nums[i]);
            total += id * sum;
            digit += nums[i];

            // println!("- {} {}", id, nums[i]);
        } else {
            while i <= last_id * 2 && nums[i] > 0 {
                let filling = min(nums[last_id * 2], nums[i]);
                let sum = sum_range(digit, filling);
                total += last_id as u64 * sum;
                digit += filling;
                nums[i] -= filling;
                nums[last_id * 2] -= filling;

                // println!("+ {} {}", last_id, filling);
                while i <= last_id * 2 && nums[last_id * 2] <= 0 {
                    last_id -= 1;
                }
            }
        }

        // println!("total = {}", total);
        i += 1;
    }
    total
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
    let mut total: u64 = 0;
    let nums = input
        .chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut filled = vec![0; nums.len()];

    'outer: for i in (0..nums.len()).rev() {
        if i % 2 == 0 {
            let mut pos = 0;
            let id = i as u64 / 2;
            for j in 0..i {
                if j % 2 == 1 {
                    if nums[i] + filled[j] <= nums[j] {
                        total += id * sum_range(pos + filled[j], nums[i]);
                        filled[j] += nums[i];
                        continue 'outer;
                    }
                }
                pos += nums[j];
            }
            total += id * sum_range(pos, nums[i]);
        }
    }

    total
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
2333133121414131402";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 1928);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 2858);
    }
}
