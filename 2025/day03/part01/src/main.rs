use rayon::prelude::*;

fn solve(input: String) {
    let total_joltage: u64 = input
        .trim()
        .par_lines()
        .map(|line| {
            let digits: Vec<u8> = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u8)
                .collect();
            let mut joltage: Option<u8> = None;

            for first in 0..digits.len() {
                for second in first + 1..digits.len() {
                    let current = digits[first] * 10 + digits[second];
                    if let Some(previous) = joltage {
                        if current > previous {
                            joltage = Some(current);
                        }
                    } else {
                        joltage = Some(current);
                    }
                }
            }

            return joltage.unwrap() as u64;
        })
        .sum();

    println!("Joltage: {total_joltage}");
}

fn main() {
    let input_sample = String::from(include_str!("../../input/input.sample.txt"));
    let input_real = String::from(include_str!("../../input/input.real.txt"));

    print!("[Sample] ");
    solve(input_sample);
    print!("[Real] ");
    solve(input_real);
}
