pub fn part1(input: &str) {
    let instrutions = input.split('\n');
    let mut position = 50;
    let mut count = 0;
    for instruction in instrutions {
        let (direction, distance) = instruction.split_at(1);
        let distance: i32 = distance.trim().parse().unwrap();
        match direction {
            "L" => {position = (position - distance) % 100;},
            "R" => {position = (position + distance) % 100;},
            _ => {},
        }
        if position == 0 {
            count += 1;
        }
    };
    print!("Part 1 result: {}\n", count);
}

pub fn part2(input: &str) {
    let instructions = input.trim().lines();
    let mut position: i64 = 50;
    let mut count: i64 = 0;

    for instruction in instructions {
        if instruction.is_empty() { continue; }
        let (direction, distance_str) = instruction.split_at(1);
        let distance: i64 = distance_str.trim().parse().unwrap();
        let old_pos = position;

        match direction {
            "L" => {position -= distance},
            "R" => {position += distance},
            _ => {},
        }

        let new_pos = position;

        if new_pos > old_pos {
            // Right rotation: (old_pos, new_pos]
            // Count multiples of 100
            count += new_pos.div_euclid(100) - old_pos.div_euclid(100);
        } else {
            // Left rotation: [new_pos, old_pos)
            // We want to count how many multiples of 100 are in the range.
            // Since we are moving downwards, we cross a multiple X if we go from >= X to < X.
            // This is equivalent to counting multiples in (new_pos - 1, old_pos - 1]
            count += (old_pos - 1).div_euclid(100) - (new_pos - 1).div_euclid(100);
        }
    }
    println!("Part 2 result: {}", count);
}

pub fn solve(input: &str) {
    part1(input);
    part2(input);
}