use rayon::prelude::*;

fn solve(input: String) {
    let total_joltage: u64 = input
        .trim()
        .par_lines()
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();
            let mut remove = line.len() - 12;

            for c in line.chars() {
                while remove > 0 && !stack.is_empty() && stack.last().unwrap() < &c {
                    stack.pop();
                    remove -= 1;
                }
                stack.push(c);
            }

            stack.truncate(12);
            return stack
                .into_iter()
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
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
