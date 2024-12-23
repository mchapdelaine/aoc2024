use std::collections::HashMap;
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

fn read_file(path: String) -> Result<(Vec<i64>, Vec<i64>), FileError> {
    let mut column1: Vec<i64> = Vec::new();
    let mut column2: Vec<i64> = Vec::new();

    match read_lines(path) {
        Ok(lines) => {
            for line in lines {
                if let Ok(row) = line {
                    let numbers: Vec<&str> = row.split_whitespace().collect();
                    if numbers.len() == 2 {
                        if let (Ok(num1), Ok(num2)) =
                            (numbers[0].parse::<i64>(), numbers[1].parse::<i64>())
                        {
                            column1.push(num1);
                            column2.push(num2);
                        } else {
                            return Err(FileError::ParsingError);
                        }
                    }
                }
            }
            Ok((column1, column2))
        }
        Err(_) => Err(FileError::ParsingError),
    }
}

fn part_1(column1: &mut Vec<i64>, column2: &mut Vec<i64>) -> Result<i64, String> {
    // Sort columns
    column1.sort();
    column2.sort();

    let distance: i64 = column1
        .iter()
        .zip(column2.iter()) // Create a pair
        .map(|(x, y)| (x - y).abs()) // Sum the absolute difference between the 2
        .sum(); // Sum the iterator

    Ok(distance)
}

fn part_2(column1: &mut Vec<i64>, column2: &mut Vec<i64>) -> Result<i64, String> {
    let mut counts = HashMap::new();

    for &value in column2.iter() {
        *counts.entry(value).or_insert(0) += 1;
    }

    let mut similarity: i64 = 0;

    for &value in column1.iter() {
        if let Some(&count) = counts.get(&value) {
            similarity += count * value;
        }
    }

    Ok(similarity)
}

fn main() {
    println!("Day 1!");

    match read_file("input.txt".to_string()) {
        Ok(mut columns) => {
            let value = part_1(&mut columns.0, &mut columns.1);

            match value {
                Ok(distance) => {
                    println!("Part 1 distance is: {}", distance);
                }
                Err(e) => {
                    panic!("Part 1 input failed with : {}", e);
                }
            }

            let value2 = part_2(&mut columns.0, &mut columns.1);

            match value2 {
                Ok(similarity) => {
                    println!("Part 2 similarity is: {}", similarity);
                }
                Err(e) => {
                    panic!("Part 2 input failed with : {}", e);
                }
            }
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
    fn test_example_part1() {
        match read_file("example.txt".to_string()) {
            Ok(mut columns) => {
                let value = part_1(&mut columns.0, &mut columns.1);

                match value {
                    Ok(distance) => {
                        assert_eq!(distance, 11, "The distance doesn't match");
                    }
                    Err(e) => {
                        panic!("Part 1 example failed with : {}", e);
                    }
                }
            }
            Err(e) => match e {
                FileError::ParsingError => {
                    assert!(true, "Parsing error");
                }
            },
        }
    }

    #[test]
    fn test_example_part2() {
        match read_file("example.txt".to_string()) {
            Ok(mut columns) => {
                let value: Result<_, _> = part_2(&mut columns.0, &mut columns.1);

                match value {
                    Ok(similarity) => {
                        assert_eq!(similarity, 31, "The similarity doesn't match");
                    }
                    Err(e) => {
                        panic!("Part 1 example failed with : {}", e);
                    }
                }
            }
            Err(e) => match e {
                FileError::ParsingError => {
                    assert!(true, "Parsing error");
                }
            },
        }
    }
}
