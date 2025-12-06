fn is_invalid(num: u64) -> bool {
    let num_str = num.to_string();

    for c in 1..=num_str.len() / 2 {
        if num_str.len() % c != 0 {
            continue;
        }

        let first_chunk = &num_str[..c];
        if num_str
            .chars()
            .collect::<Vec<_>>()
            .chunks(c)
            .all(|chunk| chunk.iter().collect::<String>() == first_chunk)
        {
            return true;
        }
    }

    return false;
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
