use std::cmp::min;

use itertools::Itertools;
use regex::Regex;

fn parse_line(line: &str) -> (i64, i64) {
    let regexp = Regex::new(r".+: X[+=](\d+), Y[+=](\d+)").unwrap();
    let [x, y] = regexp
        .captures(line)
        .unwrap()
        .extract()
        .1
        .map(|x| x.parse::<i64>().unwrap());
    (x, y)
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    let mut total = 0;

    for input_group in input.split("\n\n") {
        let ((ax, ay), (bx, by), (px, py)) = input_group
            .lines()
            .map(|line| parse_line(line))
            .collect_tuple()
            .unwrap();

        if ax * by - bx * ay == 0 || bx * ay - by * ax == 0 {
            total += min((px / ax) * 3, px / bx);
            break;
        }

        if (px * by - py * bx) % (ax * by - bx * ay) != 0 {
            continue;
        }
        let x = (px * by - py * bx) / (ax * by - bx * ay);
        if (px * ay - py * ax) % (bx * ay - by * ax) != 0 {
            continue;
        }
        let y = (px * ay - py * ax) / (bx * ay - by * ax);

        total += x * 3 + y;
    }

    total
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let mut total = 0;

    for input_group in input.split("\n\n") {
        let ((ax, ay), (bx, by), (prize_x, prize_y)) = input_group
            .lines()
            .map(|line| parse_line(line))
            .collect_tuple()
            .unwrap();
        let px = prize_x + 10000000000000;
        let py = prize_y + 10000000000000;

        if ax * by - bx * ay == 0 || bx * ay - by * ax == 0 {
            total += min((px / ax) * 3, px / bx);
            break;
        }

        if (px * by - py * bx) % (ax * by - bx * ay) != 0 {
            continue;
        }
        let x = (px * by - py * bx) / (ax * by - bx * ay);
        if (px * ay - py * ax) % (bx * ay - by * ax) != 0 {
            continue;
        }
        let y = (px * ay - py * ax) / (bx * ay - by * ax);

        total += x * 3 + y;
    }

    total
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279

Button A: X+1, Y+1
Button B: X+2, Y+2
Prize: X=10, Y=10";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 485);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 5875318608913);
    }
}
