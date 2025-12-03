pub fn part1(input: &str) {
    let banks = input.lines();
    let mut sum= 0;
    for bank in banks {
        const RADIX: u32 = 10;
        let joltages: Vec<u32> = bank.chars().map(|c| c.to_digit(RADIX).expect("conversion error")).collect();
        let (lidx, lmax) =
            index_and_max(&joltages[..joltages.len().saturating_sub(1)])
                .expect("left slice empty");

        let (_idx, rmax) =
            index_and_max(&joltages[lidx.saturating_add(1) ..])
                .expect("right slice empty");
        sum += lmax * 10 + rmax;
    }
    println!("Part 1 result: {}", sum);
}

pub fn part2(input: &str) {
    let banks = input.lines();
    let mut sum: u64 = 0;
    for bank in banks {
        const RADIX: u32 = 10;
        let joltages: Vec<u32> = bank.chars().map(|c| c.to_digit(RADIX).expect("conversion error")).collect();

        let (mut idx, lmax) =
            index_and_max(&joltages[..joltages.len().saturating_sub(12)])
                .expect("left slice empty");

        sum += (lmax as u64) * 10_u64.pow(11);
        for i in (0..10).rev() {
            let slice = &joltages[idx.saturating_add(1)..joltages.len().saturating_sub(i+1)];
            let (nidx, max) = index_and_max(slice).expect("slice empty");
            sum += (max as u64) * 10_u64.pow(i.saturating_add(1) as u32);
            idx += nidx + 1;
        }
        let (_idx, rmax) =
            index_and_max(&joltages[idx.saturating_add(1) ..])
                .expect("right slice empty");
        sum += rmax as u64;

    }
    println!("Part 2 result: {}", sum);
}

pub fn solve(input: &str) {
    part1(input);
    part2(input);
}

fn index_and_max(slice: &[u32]) -> Option<(usize, u32)> {
    let mut iter = slice.iter().enumerate();
    let (mut best_idx, mut best_val) = match iter.next() {
        Some((i, v)) => (i, *v),
        None => return None,
    };

    for (i, &v) in iter {
        if v > best_val {
            best_val = v;
            best_idx = i;
        }
    }

    Some((best_idx, best_val))
}