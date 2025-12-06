use rayon::prelude::*;

fn solve(input: String) {
    let operators_are_sum: Vec<bool> = input
        .lines()
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|op| op == "+")
        .collect();
    let input: Vec<Vec<u64>> = input
        .par_lines()
        .filter_map(|line| {
            line.trim()
                .split_whitespace()
                .map(|num_str| num_str.parse().ok())
                .collect()
        })
        .collect();
    let sum: u64 = operators_are_sum
        .par_iter()
        .enumerate()
        .map(|(column_num, is_sum)| {
            if *is_sum {
                input
                    .par_iter()
                    .filter_map(|row| row.get(column_num))
                    .sum::<u64>()
            } else {
                input
                    .par_iter()
                    .filter_map(|row| row.get(column_num))
                    .product::<u64>()
            }
        })
        .sum();
    println!("Sum: {sum}");
}

fn main() {
    let input_sample = String::from(include_str!("../../input/input.sample.txt"));
    let input_real = String::from(include_str!("../../input/input.real.txt"));

    print!("[Sample] ");
    solve(input_sample);
    print!("[Real] ");
    solve(input_real);
}
