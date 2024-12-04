use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let mut total = 0;
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    println!("{:?}", re.is_match(input));
    for captures in re.captures_iter(input) {
        let (_, [a, b]) = captures.extract();
        total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }
    total
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let mut total = 0;
    let instruction_re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don\'t\(\))").unwrap();
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let mut enabled = true;
    for captures in instruction_re.captures_iter(input) {
        let (_, [instruction]) = captures.extract();
        match re.captures(instruction) {
            Some(captures) => {
                let (_, [a, b]) = captures.extract();
                if enabled {
                    total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
                }
            }
            None => {
                if do_re.is_match(instruction) {
                    enabled = true;
                } else {
                    enabled = false;
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
    const SAMPLE_INPUT2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 161);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT2), 48);
    }
}
