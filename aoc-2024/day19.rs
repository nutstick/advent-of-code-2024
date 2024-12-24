#[aoc(day19, part1)]
pub fn part1(input: &str) -> i32 {
    let (pattern_part, design_part) = input.split_once("\n\n").unwrap();
    let patterns = pattern_part.split(", ").collect::<Vec<_>>();
    let designs = design_part.lines().collect::<Vec<_>>();

    designs
        .iter()
        .filter(|design| {
            let mut possibles: Vec<bool> = vec![false; design.len() + 1];

            possibles[0] = true;

            for i in 0..design.len() {
                if !possibles[i] {
                    continue;
                }

                for pattern in patterns.iter() {
                    if design[i..].starts_with(pattern) {
                        possibles[i + pattern.len()] = true;
                    }
                }
            }

            possibles[design.len()]
        })
        .count() as i32
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> i64 {
    let (pattern_part, design_part) = input.split_once("\n\n").unwrap();
    let patterns = pattern_part.split(", ").collect::<Vec<_>>();
    let designs = design_part.lines().collect::<Vec<_>>();

    designs
        .iter()
        .map(|design| {
            let mut possibility: Vec<i64> = vec![0; design.len() + 1];

            possibility[0] = 1;

            for i in 0..design.len() {
                if possibility[i] == 0 {
                    continue;
                }

                for pattern in patterns.iter() {
                    if design[i..].starts_with(pattern) {
                        possibility[i + pattern.len()] += possibility[i];
                    }
                }
            }

            possibility[design.len()]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_sample_part_1() {
        assert_eq!(6, part1(SAMPLE_INPUT_1));
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(16, part2(SAMPLE_INPUT_1));
    }
}
