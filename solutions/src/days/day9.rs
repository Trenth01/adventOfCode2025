use graphics_shapes::{Shape, prelude::{ContainsShape, Polygon}, rect::Rect};
use itertools::Itertools;

pub fn part1(input: &str) {
    let coords = input
        .lines()
        .map(|l| {
            let mut it = l.split(',').map(|s| s.trim().parse::<i64>().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();
    let mut max_area = 0;
    for (a, b) in coords.iter().tuple_combinations() {
        let area = square_size(*a, *b);
        if area > max_area {
            max_area = area;
        }
    }
    println!("Part 1: {}", max_area);

}

pub fn part2(input: &str) {
    let coords = input
        .lines()
        .map(|l| {
            let mut it = l.split(',').map(|s| s.trim().parse::<f32>().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect::<Vec<(f32, f32)>>();
    let polygon = Polygon::new(&coords);
    let mut max_area = 0i64;
    for (a, b) in coords.iter().tuple_combinations() {
        // floor to integers and get ordered corners
        let ax = a.0.floor() as i64;
        let ay = a.1.floor() as i64;
        let bx = b.0.floor() as i64;
        let by = b.1.floor() as i64;

        let x_min = ax.min(bx);
        let x_max = ax.max(bx);
        let y_min = ay.min(by);
        let y_max = ay.max(by);

        // integer cell area (inclusive)
        let area = (x_max - x_min + 1) * (y_max - y_min + 1);

        // build a Rect in float space that covers those integer cells
        let rect_min = (x_min as f32, y_min as f32);
        let rect_max = ((x_max + 1) as f32, (y_max + 1) as f32);
        let inner_rect = Rect::new(rect_min, rect_max);

        if area > max_area && polygon.contains_rect(&inner_rect) {
            max_area = area;
        }
    }
    println!("Part 2: {}", max_area);
}

pub fn solve(input: &str) {
    part1(input);
    part2(input);
}

fn square_size(a: (i64, i64), b: (i64, i64)) -> i64 {
    let dx = (a.0 - b.0).abs().strict_add(1);
    let dy = (a.1 - b.1).abs().strict_add(1);
    return dx * dy;
}