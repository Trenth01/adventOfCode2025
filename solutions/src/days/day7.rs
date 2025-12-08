pub fn part1(input: &str) {
    let rows = input.lines();

    let mut current = rows.clone()
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let mut beam = <Vec<char>>::new();
    let mut split_count = 0;

    for (i, row) in rows.enumerate() {
        if i % 2 == 1 {
            continue;
        } else if i == 0 {
            beam = current.iter().map(|&c| if c == 'S' { '|' } else { c }).collect();
            continue
        }else{
            current = row.chars().collect::<Vec<char>>();
        }
        let mut new_beam = beam.clone();
        for(j, c) in current.iter().enumerate() {
            if c == &'^' && beam[j] == '|' {
                split_count += 1;
                new_beam[j] = '.';
                if j > 0 {
                    new_beam[j-1] = '|';
                } 
                if j+1 < beam.len() {
                    new_beam[j+1] = '|';
                }
            }
        }
        beam = new_beam;
    }
    let _beam_size: String = beam.iter().filter(|&c| c == &'|').count().to_string();
    println!("Part 1 result: {}", split_count);
}

pub fn part2(input: &str) {
    let rows = input.lines();

    let mut current = rows.clone()
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let mut beam = <Vec<char>>::new();
    let mut beam_counts: Vec<i64> = vec![0; current.len()];



    for (i, row) in rows.enumerate() {
        if i % 2 == 1 {
            continue;
        } else if i == 0 {
            beam = current.iter().map(|&c| if c == 'S' { '|' } else { c }).collect();
            for (j, c) in beam.iter().enumerate() {
                if c == &'|' {
                    beam_counts[j] += 1;
                }
            }
            continue
        }else{
            current = row.chars().collect::<Vec<char>>();
        }
        let mut new_beam = beam.clone();
        let mut new_beam_counts = beam_counts.clone();
        for(j, c) in current.iter().enumerate() {
            if c == &'^' && beam[j] == '|' {
                new_beam[j] = '.';
                if j > 0 {
                    new_beam[j-1] = '|';
                    new_beam_counts[j-1] += beam_counts[j];
                    new_beam_counts[j] = 0;
                } 
                if j+1 < beam.len() {
                    new_beam[j+1] = '|';
                    new_beam_counts[j+1] += beam_counts[j];
                    new_beam_counts[j] = 0;
                }
            }
        }
        beam = new_beam;
        beam_counts = new_beam_counts;
    }
    println!("Part 2 result: {}", beam_counts.iter().sum::<i64>());
}

pub fn solve(input: &str) {
    part1(input);
    part2(input);
}
