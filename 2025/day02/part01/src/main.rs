fn is_invalid(num: u64) -> bool {
    let num_str = num.to_string();
    return &num_str[..num_str.len() / 2] == &num_str[num_str.len() / 2..];
}

fn solve(input: String) {
    let mut sum = 0;

    for range in input.trim().split_terminator(',') {
        let (start, end): (u64, u64) = range
            .split_once('-')
            .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
            .unwrap();
        for num in start..=end {
            if is_invalid(num) {
                sum += num;
            }
        }
    }

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
