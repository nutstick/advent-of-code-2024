const XMAS: &str = "XMAS";

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let dir: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (-1, 1),
    ];

    let board = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut count = 0;
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            for (x, y) in dir.iter() {
                let mut found = true;
                for word_index in 0..XMAS.len() {
                    let tx = i as i32 - x * (word_index as i32);
                    if tx < 0 {
                        found = false;
                        break;
                    }
                    match board.get(tx as usize) {
                        Some(row) => {
                            let ty = j as i32 - y * (word_index as i32);
                            if ty < 0 {
                                found = false;
                                break;
                            }
                            match row.get(ty as usize) {
                                Some(c) => {
                                    if *c != XMAS.chars().nth(word_index).unwrap() {
                                        found = false;
                                        break;
                                    }
                                }
                                None => {
                                    found = false;
                                    break;
                                }
                            }
                        }
                        None => {
                            found = false;
                            break;
                        }
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }
    count
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let board = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut count = 0;
    let dir: Vec<(i32, i32)> = vec![(-1, -1), (-1, 1), (1, 1), (1, -1)];
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 'A' {
                for idx in 0..dir.len() {
                    let mut found = true;
                    for k in 0..2 {
                        let (x, y) = dir[(idx + k as usize) % dir.len()];
                        if (i as i32) + x >= 0 {
                            match board.get(((i as i32) + x) as usize) {
                                Some(row) => {
                                    if (j as i32) + y >= 0 {
                                        match row.get(((j as i32) + y) as usize) {
                                            Some('M') => {}
                                            _ => {
                                                found = false;
                                                break;
                                            }
                                        }
                                    } else {
                                        found = false;
                                        break;
                                    }
                                }
                                _ => {
                                    found = false;
                                    break;
                                }
                            }
                        } else {
                            found = false;
                            break;
                        }
                    }
                    if !found {
                        continue;
                    }
                    for k in 2..4 {
                        let (x, y) = dir[(idx + k as usize) % dir.len()];
                        if (i as i32) + x >= 0 {
                            match board.get(((i as i32) + x) as usize) {
                                Some(row) => {
                                    if (j as i32) + y >= 0 {
                                        match row.get(((j as i32) + y) as usize) {
                                            Some(c) => {
                                                if *c != 'S' {
                                                    found = false;
                                                    break;
                                                }
                                            }
                                            None => {
                                                found = false;
                                                break;
                                            }
                                        }
                                    } else {
                                        found = false;
                                        break;
                                    }
                                }
                                None => {
                                    found = false;
                                    break;
                                }
                            }
                        } else {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT), 18);
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT), 9);
    }
}
