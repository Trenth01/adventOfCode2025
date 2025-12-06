pub fn part1(input: &str) {
    let (raw_ranges,  raw_ingredients) = input.split_once("\r\n\r\n").unwrap();
    let ranges: Vec<(i64, i64)> = raw_ranges
        .split("\r\n")
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<i64>().ok()?;
                let end = parts[1].parse::<i64>().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect();
    
    let ingredients: Vec<i64> = raw_ingredients
        .split("\r\n")
        .filter_map(|line| line.trim().parse::<i64>().ok())
        .collect();

    let mut valid_ingredients: Vec<i64> = Vec::new();
    
    for ingredient in ingredients {
        for (start, end) in &ranges {
            if ingredient >= *start && ingredient <= *end {
                valid_ingredients.push(ingredient);
                break;
            }
        }
    }
    print!("Part 1 result: {}\n", valid_ingredients.len());

}

pub fn part2(input: &str) {
let (raw_ranges,  _raw_ingredients) = input.split_once("\r\n\r\n").unwrap();
    let mut ranges: Vec<(i64, i64)> = raw_ranges
        .split("\r\n")
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<i64>().ok()?;
                let end = parts[1].parse::<i64>().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect();
    ranges.sort_by_key(|r| r.0);
    let mut used_ranges: Vec<(i64, i64)> = Vec::new();
    for (start, end) in &ranges {
        if used_ranges.is_empty() {
            used_ranges.push((*start, *end));
        } else {
            let last = used_ranges.last_mut().unwrap();
            if *start <= last.1 + 1 {
                // Overlapping or adjacent, merge
                last.1 = last.1.max(*end);
            } else {
                // No overlap, add new range
                used_ranges.push((*start, *end));
            }
        }
    }

    let mut total_range: i64 = 0;
    for (start, end) in &used_ranges {
        total_range += end - start + 1;
    }
    print!("Part 2 result: {}\n", total_range);
}

pub fn solve(input: &str) {
    part1(input);
    part2(input);
}
