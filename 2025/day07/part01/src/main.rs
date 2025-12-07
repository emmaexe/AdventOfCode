use std::collections::HashSet;

fn solve(input: &str) {
    let mut beams: HashSet<usize> = HashSet::with_capacity(input.find('\n').unwrap());
    beams.insert(input.find('S').unwrap());

    let split_count: u64 = input
        .lines()
        .skip(1)
        .filter(|line| line.contains('^'))
        .map(|line| {
            let mut split_count = 0;
            let splitters: Vec<bool> = line.chars().map(|c| c == '^').collect();
            let mut next_beams = HashSet::with_capacity(beams.capacity());

            for beam in &beams {
                if splitters[*beam] {
                    split_count += 1;
                    next_beams.insert(*beam - 1);
                    next_beams.insert(*beam + 1);
                } else {
                    next_beams.insert(*beam);
                }
            }

            beams = next_beams;
            return split_count;
        })
        .sum();

    println!("Split count: {split_count}");
}

fn main() {
    print!("[Sample] ");
    solve(include_str!("../../input/input.sample.txt"));
    print!("[Real] ");
    solve(include_str!("../../input/input.real.txt"));
}
