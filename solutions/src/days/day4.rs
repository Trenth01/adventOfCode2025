pub fn part1(input: &str) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for x in 0..rows {
        for y in 0..cols {
            let cell = grid[x][y];
            let neighbors = num_neighbors(x as isize, y as isize, &grid);

            if cell == '@' && neighbors < 4 {
                count+=1;
            } 
        }
    }

    println!("Part 1 result: {}", count);

}

pub fn part2(input: &str) {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();


    let mut count = 0;
    let mut it_count = -1;

    while it_count != 0 {
        it_count = 0;
        for x in 0..rows {
            for y in 0..cols {
                let cell = grid[x][y];
                let neighbors = num_neighbors(x as isize, y as isize, &grid);

                if cell == '@' && neighbors < 4 {
                    it_count+=1;
                    grid[x][y] = '.';
                } 
            }
        }
        count += it_count;
    }

    println!("Part 2 result: {}", count);

}

pub fn solve(input: &str) {
    part1(input);
    part2(input);
}

fn num_neighbors(x: isize, y: isize, grid: &Vec<Vec<char>>) -> usize {
    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for (dx, dy) in deltas.iter() {
        let nx = x + dx;
        let ny = y + dy;

        if nx >= 0 && nx < rows && ny >= 0 && ny < cols {
            if grid[nx as usize][ny as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}
