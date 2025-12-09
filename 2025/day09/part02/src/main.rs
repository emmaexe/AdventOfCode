use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

struct Point {
    x: i64,
    y: i64,
}

struct Rectangle {
    corner_0_idx: usize,
    corner_1_idx: usize,
    area: i64,
}

fn solve(input: &str) {
    let coords: Vec<Point> = input
        .lines()
        .map(|line| {
            let (x_str, y_str) = line.split_once(',').unwrap();
            Point {
                x: x_str.parse().unwrap(),
                y: y_str.parse().unwrap(),
            }
        })
        .collect();

    let mut rectangles: Vec<Rectangle> = Vec::with_capacity(coords.len() * (coords.len() + 1) / 2);
    for i in 0..coords.len() - 1 {
        for j in i + 1..coords.len() {
            rectangles.push(Rectangle {
                corner_0_idx: i,
                corner_1_idx: j,
                area: ((coords[i].x - coords[j].x).abs() + 1)
                    * ((coords[i].y - coords[j].y).abs() + 1),
            });
        }
    }
    rectangles.sort_unstable_by(|a, b| b.area.cmp(&a.area));

    let mut x_only: Vec<i64> = coords.iter().map(|p| p.x).collect();
    x_only.sort_unstable();
    x_only.dedup();
    let mut y_only: Vec<i64> = coords.iter().map(|p| p.y).collect();
    y_only.sort_unstable();
    y_only.dedup();

    let x_map: HashMap<i64, usize> = x_only.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let y_map: HashMap<i64, usize> = y_only.iter().enumerate().map(|(i, &v)| (v, i)).collect();

    let new_coords: Vec<Point> = coords
        .iter()
        .map(|p| Point {
            x: 2 * x_map[&p.x] as i64,
            y: 2 * y_map[&p.y] as i64,
        })
        .collect();

    let x_min = new_coords.iter().map(|p| p.x).min().unwrap();
    let x_max = new_coords.iter().map(|p| p.x).max().unwrap();
    let y_min = new_coords.iter().map(|p| p.y).min().unwrap();
    let y_max = new_coords.iter().map(|p| p.y).max().unwrap();

    let w = (x_max - x_min + 1) as usize;
    let h = (y_max - y_min + 1) as usize;

    let mut matrix = vec![vec![false; w]; h];
    let mut set = |x: i64, y: i64| {
        matrix[(y - y_min) as usize][(x - x_min) as usize] = true;
    };

    for i in 0..new_coords.len() {
        let p1 = &new_coords[i];
        let p2 = &new_coords[(i + 1) % new_coords.len()];

        if p1.x == p2.x {
            let (y1, y2) = if p1.y <= p2.y {
                (p1.y, p2.y)
            } else {
                (p2.y, p1.y)
            };
            for y in y1..=y2 {
                set(p1.x, y);
            }
        } else {
            let (x1, x2) = if p1.x <= p2.x {
                (p1.x, p2.x)
            } else {
                (p2.x, p1.x)
            };
            for x in x1..=x2 {
                set(x, p1.y);
            }
        }
    }

    for row in matrix.iter_mut() {
        let mut inside = false;
        let mut x = 0;
        while x < w {
            if row[x] {
                inside = !inside;
                while x + 1 < w && row[x + 1] {
                    x += 1;
                }
            } else if inside {
                row[x] = true;
            }
            x += 1;
        }
    }

    let area: i64 = rectangles
        .par_iter()
        .find_first(|rectangle| {
            let p0 = &new_coords[rectangle.corner_0_idx];
            let p1 = &new_coords[rectangle.corner_1_idx];

            let x1 = p0.x.min(p1.x);
            let x2 = p0.x.max(p1.x);
            let y1 = p0.y.min(p1.y);
            let y2 = p0.y.max(p1.y);

            // This is slow (should be swapped out for a rectangle border check) but I can't be bothered at this point
            for yy in y1..=y2 {
                for xx in x1..=x2 {
                    if !matrix[yy as usize][xx as usize] {
                        return false;
                    }
                }
            }
            return true;
        })
        .unwrap()
        .area;

    println!("Area: {area}");
}

fn main() {
    print!("[Sample] ");
    solve(include_str!("../../input/input.sample.txt"));
    print!("[Real] ");
    solve(include_str!("../../input/input.real.txt"));
}
