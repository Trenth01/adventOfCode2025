use std::collections::{HashSet};

pub fn part1(input: &str) {
    let ids = input.split(',');
    let mut total = 0;
    for id in ids {
        let mut parts = id.split('-');
        let start = parts.next().unwrap();
        let end = parts.next().unwrap();
        let s: i64 = start.trim().parse::<i64>().unwrap();
        let e = end.trim().parse::<i64>().unwrap();
        for i in s..=e {
            if is_invalid_p1(i) {
                total += i;
            }
        }
    }
    println!("Part 1 result: {}", total);
}

pub fn part2(input: &str) {
    let ids = input.split(',');
    let mut total = 0;
    for id in ids {
        let mut parts = id.split('-');
        let start = parts.next().unwrap();
        let end = parts.next().unwrap();
        let s: i64 = start.trim().parse::<i64>().unwrap();
        let e = end.trim().parse::<i64>().unwrap();
        for i in s..=e {
            if is_invalid_p2(i) {
                total += i;
            }
        }
    }
    println!("Part 2 result: {}", total);
}

pub fn solve(input: &str) {
    part1(input);
    part2(input);
}

fn is_invalid_p1(id:i64) -> bool {
    let digits: Vec<u32> = id.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
    if digits.len() % 2 != 0 {
        return false; // valid automatically if odd length
    }
    let firsthalf = &digits[..digits.len()/2];
    let secondhalf = &digits[digits.len()/2..];
    return firsthalf == secondhalf;
}

fn is_invalid_p2(id:i64) -> bool {
    let digits: Vec<u32> = id.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
    if digits.len() <= 1 {
        return false; // single-digit is automatically valid
    }

    let unique: HashSet<u32> = digits.iter().cloned().collect();
    if unique.len() == 1 {
        return true; // invalid if all digits are the same
    }

    let n = digits.len();
    for k in 2..=(n/2) {
        if n % k != 0 { continue; }
        let mut all_chunks = HashSet::new();
        for chunk in digits.chunks(k) {
            all_chunks.insert(chunk);
        }
        if all_chunks.len() == 1 {
            return true; // invalid if all chunks are the same
        }
    }
    return false
}