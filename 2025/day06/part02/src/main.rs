use rayon::prelude::*;

fn solve(input: String) {
    let split_positions: Vec<usize> = input
        .lines()
        .last()
        .unwrap()
        .char_indices()
        .skip(1)
        .filter_map(|(i, c)| if !c.is_whitespace() { Some(i) } else { None })
        .collect();

    let matrix: Vec<Vec<&str>> = input
        .par_lines()
        .map(|line| {
            let mut split_line: Vec<&str> = Vec::with_capacity(split_positions.len() + 1);
            let mut start = 0;

            for split_pos in &split_positions {
                split_line.push(&line[start..(*split_pos - 1)]);
                start = *split_pos;
            }
            split_line.push(&line[start..]);

            return split_line;
        })
        .collect();
    let matrix: Vec<Vec<&str>> = (0..matrix[0].len())
        .map(|idx| matrix.par_iter().map(|row| row[idx]).collect())
        .collect();

    let sum: u64 = matrix
        .par_iter()
        .map(|expression| {
            let expr_iter = expression.iter().rev().skip(1).rev();
            let mut real_numbers: Vec<u64> = Vec::with_capacity(expression.first().unwrap().len());

            for idx in 0..real_numbers.capacity() {
                real_numbers.push(
                    expr_iter
                        .clone()
                        .filter_map(|s| s.chars().nth(idx))
                        .collect::<String>()
                        .trim()
                        .parse()
                        .unwrap(),
                );
            }

            if expression.last().unwrap().starts_with("+") {
                real_numbers.iter().sum::<u64>()
            } else {
                real_numbers.iter().product::<u64>()
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
