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
    let mut sum: u64 = 0;

    for bank in input.lines() {
        const RADIX: u32 = 10;
        let joltages: Vec<u32> =
            bank.chars().map(|c| c.to_digit(RADIX).unwrap()).collect();

        let total = joltages.len();

        // FIX: start at -1 without using usize::MAX
        let mut idx: isize = -1;

        let mut digits_left = 12;
        let mut value: u64 = 0;

        for _pos in 0..12 {
            let pick_left = digits_left - 1;

            // FIX: compute bounds safely using signed indexing
            let start = (idx + 1) as usize;
            let end = total - pick_left;

            let (rel, maxdigit) = index_and_max(&joltages[start..end]).unwrap();

            // FIX: update idx safely
            idx = (start + rel) as isize;

            // FIX: avoid overflow using u64
            value = value * 10 + (maxdigit as u64);

            digits_left -= 1;
        }

        sum += value;
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