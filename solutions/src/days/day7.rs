pub fn part1(input: &str) {
    let rows: Vec<&str> = input.lines().collect();
    let mut current: Vec<char> = rows[0].chars().collect();
    let mut beam: Vec<char> = current.iter().map(|&c| if c == 'S' { '|' } else { c }).collect();

    let mut split_count = 0;
    let mut next_beam = vec!['.'; current.len()];

    for row in rows.iter().skip(2).step_by(2) {
        current = row.chars().collect();
        next_beam.copy_from_slice(&beam);

        for (j, c) in current.iter().enumerate() {
            if *c == '^' && beam[j] == '|' {
                split_count += 1;
                next_beam[j] = '.';
                if j > 0     { next_beam[j - 1] = '|'; }
                if j + 1 < current.len() { next_beam[j + 1] = '|'; }
            }
        }

        std::mem::swap(&mut beam, &mut next_beam);
    }

    println!("Part 1 result: {}", split_count);
}

pub fn part2(input: &str) {
    let rows: Vec<&str> = input.lines().collect();
    let mut current: Vec<char> = rows[0].chars().collect();
    let width = current.len();

    let mut beam: Vec<char> = current.iter().map(|&c| if c == 'S' { '|' } else { c }).collect();
    let mut beam_counts = vec![0_i64; width];

    for j in 0..width {
        if beam[j] == '|' {
            beam_counts[j] = 1;
        }
    }

    let mut next_beam = vec!['.'; width];
    let mut next_counts = vec![0_i64; width];

    for row in rows.iter().skip(2).step_by(2) {
        current = row.chars().collect();

        next_beam.copy_from_slice(&beam);
        next_counts.fill(0);

        for (j, c) in current.iter().enumerate() {
            if *c == '^' && beam[j] == '|' {
                let count = beam_counts[j];
                next_beam[j] = '.';

                if j > 0 {
                    next_beam[j - 1] = '|';
                    next_counts[j - 1] += count;
                }
                if j + 1 < width {
                    next_beam[j + 1] = '|';
                    next_counts[j + 1] += count;
                }

                next_counts[j] = 0;
            } else {
                next_counts[j] += beam_counts[j];
            }
        }

        std::mem::swap(&mut beam, &mut next_beam);
        std::mem::swap(&mut beam_counts, &mut next_counts);
    }

    println!("Part 2 result: {}", beam_counts.iter().sum::<i64>());
}

pub fn solve(input: &str) {
    part1(input);
    part2(input);
}
