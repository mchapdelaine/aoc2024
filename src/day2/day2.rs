use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum FileError {
    ParsingError,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(path: String) -> Result<Vec<Vec<u32>>, FileError> {
    let mut report: Vec<Vec<u32>> = vec![];
    match read_lines(path) {
        Ok(lines) => {
            for line in lines {
                if let Ok(row) = line {
                    let levels_string: Vec<&str> = row.split_whitespace().collect();
                    let mut levels: Vec<u32> = vec![];
                    for level in levels_string {
                        if let Ok(num) = level.parse::<u32>() {
                            levels.push(num)
                        }
                    }
                    report.push(levels);
                }
            }
            Ok(report)
        }
        Err(_) => Err(FileError::ParsingError),
    }
}

#[derive(PartialEq, Debug)]
enum LevelState {
    Increase,
    Decrease,
}

fn is_level_safe_part1(level: &Vec<u32>) -> bool {
    let mut current_value = level[0];
    let state: LevelState;

    if level[0] > level[1] {
        state = LevelState::Decrease;
    } else if level[0] < level[1] {
        state = LevelState::Increase;
    } else {
        // If the 2 first are the same value, it's not safe
        return false;
    }

    for i in 1..level.len() {
        let next_value = level[i];

        match state {
            LevelState::Increase => {
                if (next_value < current_value) || (next_value == current_value) {
                    // Decreasing or same value, not safe
                    return false;
                } else if (next_value - current_value) <= 3 {
                    // Safe, change value and continue
                    current_value = next_value;
                } else {
                    // More than 3, unsafe
                    return false;
                }
            }
            LevelState::Decrease => {
                if (next_value > current_value) || (next_value == current_value) {
                    // Increasing or same value, not safe
                    return false;
                } else if (current_value - next_value) <= 3 {
                    // Safe, change value and continue
                    current_value = next_value;
                } else {
                    return false;
                }
            }
        }
    }

    true
}

fn is_level_safe_part2(level: &Vec<u32>) -> bool {
    for i in 0..level.len() {
        let mut new_level: Vec<u32> = level.to_vec();
        new_level.remove(i);

        if is_level_safe_part1(&new_level) {
            return true;
        }
    }

    return false;
}

fn part_1(reports: &Vec<Vec<u32>>) -> u32 {
    let mut valids: u32 = 0;

    for level in reports {
        if is_level_safe_part1(level) {
            valids += 1;
        }
    }

    valids
}

fn part_2(reports: &Vec<Vec<u32>>) -> u32 {
    let mut valids: u32 = 0;

    for level in reports {
        if is_level_safe_part2(level) {
            valids += 1;
        }
    }

    valids
}

fn main() {
    println!("Day 2");

    match read_file("input.txt".to_string()) {
        //match read_file("example.txt".to_string()) {
        Ok(mut reports) => {
            let value_part1 = part_1(&mut reports);
            let value_part2 = part_2(&mut reports);

            println!("Part1: Got safe reports: {}", value_part1);
            println!("Part2: Got safe reports: {}", value_part2);
        }
        Err(e) => match e {
            FileError::ParsingError => {
                assert!(true, "Parsing error");
            }
        },
    }
}

#[cfg(test)]
mod tests {
    // Import names
    use super::*;

    #[test]
    fn test_day2_part1() {
        let safe1: Vec<u32> = vec![7, 6, 4, 2, 1];
        let unsafe1: Vec<u32> = vec![1, 2, 7, 8, 9];
        let unsafe2: Vec<u32> = vec![9, 7, 6, 2, 1];
        let unsafe3: Vec<u32> = vec![1, 3, 2, 4, 5];
        let unsafe4: Vec<u32> = vec![8, 6, 4, 4, 1];
        let safe2: Vec<u32> = vec![1, 3, 6, 7, 9];

        assert_eq!(true, is_level_safe_part1(&safe1));
        assert_eq!(true, is_level_safe_part1(&safe2));
        assert_eq!(false, is_level_safe_part1(&unsafe1));
        assert_eq!(false, is_level_safe_part1(&unsafe2));
        assert_eq!(false, is_level_safe_part1(&unsafe3));
        assert_eq!(false, is_level_safe_part1(&unsafe4));
    }

    #[test]
    fn test_day2_part2() {
        let safe1: Vec<u32> = vec![7, 6, 4, 2, 1];
        let unsafe1: Vec<u32> = vec![1, 2, 7, 8, 9];
        let unsafe2: Vec<u32> = vec![9, 7, 6, 2, 1];
        let safe2: Vec<u32> = vec![1, 3, 2, 4, 5];
        let safe3: Vec<u32> = vec![8, 6, 4, 4, 1];
        let safe4: Vec<u32> = vec![1, 3, 6, 7, 9];

        assert_eq!(true, is_level_safe_part2(&safe1));
        assert_eq!(true, is_level_safe_part2(&safe2));
        assert_eq!(true, is_level_safe_part2(&safe3));
        assert_eq!(true, is_level_safe_part2(&safe4));
        assert_eq!(false, is_level_safe_part2(&unsafe1));
        assert_eq!(false, is_level_safe_part2(&unsafe2));
    }
}
