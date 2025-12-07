fn solve(input: &str) {
    let mut beams: Vec<u64> = vec![0; input.find('\n').unwrap()];
    beams[input.find('S').unwrap()] = 1;

    input
        .lines()
        .skip(1)
        .filter(|line| line.contains('^'))
        .for_each(|line| {
            let mut next_beams = beams.clone();
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'^')
                .map(|(i, _)| i)
                .for_each(|splitter| {
                    if beams[splitter] > 0 {
                        next_beams[splitter] = 0;
                        if let Some(prev) = splitter.checked_sub(1) {
                            next_beams[prev] += beams[splitter];
                        }
                        if let Some(next) = splitter.checked_add(1) {
                            next_beams[next] += beams[splitter];
                        }
                    }
                });
            beams = next_beams;
        });

    let timeline_count: u64 = beams.iter().sum();

    println!("Timeline count: {timeline_count}");
}

fn main() {
    print!("[Sample] ");
    solve(include_str!("../../input/input.sample.txt"));
    print!("[Real] ");
    solve(include_str!("../../input/input.real.txt"));
}
