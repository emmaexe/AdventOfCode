use rayon::prelude::*;

fn solve(input: &str) {
    let coords: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            let (x_str, y_str) = line.split_once(',').unwrap();
            (x_str.parse().unwrap(), y_str.parse().unwrap())
        })
        .collect();
    let area: i64 = (0..coords.len() - 1)
        .into_par_iter()
        .map(|i| {
            (i + 1..coords.len())
                .into_par_iter()
                .map(|j| {
                    ((coords[i].0 - coords[j].0).abs() + 1)
                        * ((coords[i].1 - coords[j].1).abs() + 1)
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    println!("Area: {area}");
}

fn main() {
    print!("[Sample] ");
    solve(include_str!("../../input/input.sample.txt"));
    print!("[Real] ");
    solve(include_str!("../../input/input.real.txt"));
}
