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
    let instrutions = input.split('\n');
    let mut position: i32 = 50;
    let mut count = 0;
    for instruction in instrutions {
        let old_sign: i32 = position.signum();
        let (direction, distance) = instruction.split_at(1);
        let distance: i32 = distance.trim().parse().unwrap();
        match direction {
            "L" => {position -= distance},
            "R" => {position += distance},
            _ => {},
        }
        let new_sign: i32 = position.signum();
        if position <= 0 || position >= 100 {
            let clicks = (position / 100).abs();
            count += (position / 100).abs();
            if (old_sign != new_sign) && (position % 100 != 0) {
                count += 1;
            }
        }
        position %= 100;
    };
    print!("Part 2 result: {}\n", count);
}

pub fn solve(input: &str) {
    part1(input);
    part2(input);
}
