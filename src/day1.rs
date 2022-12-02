use std::fs;

pub fn part1() {
    let str = really_read_to_string("day1_part1.txt");
    println!("{}", split(str, 1));
}

pub fn part2() {
    let str = really_read_to_string("day1_part1.txt");
    println!("{}", split(str, 3));
}

fn really_read_to_string(file: &str) -> String {
    let text: std::io::Result<String> = fs::read_to_string(file);
    let string = text.expect("fail");
    string
}

fn split(text: String, n: usize) -> i32 {
    let mut sums: Vec<i32> = text
        .split("\n\n")
        .map(|it: &str| it.lines().map(|line: &str| line.parse::<i32>().unwrap()))
        .map(|elf| elf.sum())
        .collect();

    sums.sort();
    sums.reverse();

    return sums.iter().take(n).sum::<i32>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let str = really_read_to_string("day1_test.txt");

        assert_eq!(split(str, 1), 24000);
    }

    #[test]
    fn test_part2() {
        let str = really_read_to_string("day1_test.txt");

        assert_eq!(split(str, 3), 45000);
    }
}