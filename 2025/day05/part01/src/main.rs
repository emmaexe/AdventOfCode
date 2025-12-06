use rayon::prelude::*;

fn ranges_overlap((start1, end1): &(usize, usize), (start2, end2): &(usize, usize)) -> bool {
    start2 <= start1 && start1 <= end2
        || start2 <= end1 && end1 <= end2
        || start1 <= start2 && start2 <= end1
        || start1 <= end2 && end2 <= end1
}

fn solve(input: String) {
    let (raw_ranges, raw_availables) = input.split_once("\n\n").unwrap();

    let mut all_ranges: Vec<(usize, usize)> = raw_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            return (start.parse().unwrap(), end.parse().unwrap());
        })
        .collect();
    all_ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut ranges: Vec<(usize, usize)> = Vec::with_capacity(all_ranges.len());
    assert!(all_ranges.len() > 0);
    ranges.push(all_ranges[0]);
    for current_range in all_ranges.iter().skip(1) {
        let last_range = ranges.last_mut().unwrap();
        if ranges_overlap(last_range, current_range) {
            last_range.1 = std::cmp::max(last_range.1, current_range.1);
        } else {
            ranges.push(*current_range);
        }
    }

    let fresh: usize = raw_availables
        .par_lines()
        .filter(|raw_available| {
            let available: usize = raw_available.parse().unwrap();
            for range in &ranges {
                if range.0 <= available && available <= range.1 {
                    return true;
                }
            }
            return false;
        })
        .count();

    println!("Fresh: {fresh}");
}

fn main() {
    let input_sample = String::from(include_str!("../../input/input.sample.txt"));
    let input_real = String::from(include_str!("../../input/input.real.txt"));

    print!("[Sample] ");
    solve(input_sample);
    print!("[Real] ");
    solve(input_real);
}
