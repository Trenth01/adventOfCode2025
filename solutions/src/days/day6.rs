use std::ffi::c_char;

pub fn part1(input: &str) {
    let rows = input.lines();
    let mut operations: Vec<Vec<String>> = Vec::new();
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut sum = 0;

    for row in rows.rev() {
        if operations.is_empty() {
            operations.push(row.split_whitespace().map(|s| s.to_string()).collect());
            continue;
        }
        let nums: Vec<i64> = row
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        numbers.push(nums);     
    }
    for col in 0..numbers[0].len() {
        let mut col_sum = 0;
        if operations[0][col] == "+" {
            for row in 0..numbers.len() {
                col_sum += numbers[row][col];
            } 
        } else {
            col_sum = 1;
            for row in 0..numbers.len() {
                col_sum *= numbers[row][col];
            }
        }
        sum += col_sum;
    }
    
    println!("Part 1 result: {}", sum);
}

pub fn part2(input: &str) {
    let rows: Vec<&str> = input.lines().collect();
    let operations: Vec<char> = rows.clone().last().unwrap().chars().collect();
    let mut sum = 0;
    for (i, c) in operations.iter().enumerate() {
        if c.is_whitespace(){
            continue;
        }
        let mut pos = i;
        let mut check_char = operations[i+1];
        let mut cell_numbers: Vec<i64> = Vec::new();
        while check_char.is_whitespace() && pos < operations.len() {
            if pos + 1 >= operations.len() {
                check_char = 'x';
            }else{
                check_char = operations[pos + 1];
            }
            let mut cephalopod_number = String::new();
            for row in &rows[..rows.len() - 1] {
                cephalopod_number.push(row.chars().nth(pos).unwrap());
            }
            if cephalopod_number.trim().is_empty() {
                break;
            }
            cell_numbers.push(cephalopod_number.trim().parse::<i64>().unwrap());
            pos += 1;
        }
        if operations[i] == '+' {
            let col_sum: i64 = cell_numbers.iter().sum();
            sum += col_sum;
        } else {
            let col_product: i64 = cell_numbers.iter().product();
            sum += col_product;
        }
    }

    println!("Part 2 result: {}", sum);
}

pub fn solve(input: &str) {
    part1(input);
    part2(input);
}
