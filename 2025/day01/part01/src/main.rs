fn solve(input: String) {
    let mut dial: u64 = 50;
    let mut password = 0;

    for line in input.lines() {
        let num: u64 = line[1..].parse().unwrap();
        if line.chars().nth(0).unwrap() == 'L' {
            while dial < num {
                dial += 100;
            }
            dial -= num;
        } else if line.chars().nth(0).unwrap() == 'R' {
            dial += num;
            while dial >= 100 {
                dial -= 100;
            }
        }

        if dial == 0 {
            password += 1;
        }
    }

    println!("Password: {password}");
}

fn main() {
    let input_sample = String::from(include_str!("../../input/input.sample.txt"));
    let input_real = String::from(include_str!("../../input/input.real.txt"));

    print!("[Sample] ");
    solve(input_sample);
    print!("[Real] ");
    solve(input_real);
}
