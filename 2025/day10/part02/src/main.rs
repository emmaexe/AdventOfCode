use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

fn solve(input: &str) {
    let machines: Vec<(Vec<bool>, Vec<Vec<bool>>, Vec<u64>)> = input
        .par_lines()
        .map(|line| {
            let mut parts = line.split_once(']').unwrap();
            let lights: Vec<bool> = parts.0.chars().skip(1).map(|c| c == '#').collect();
            parts = parts.1.split_once('{').unwrap();
            let buttons = parts
                .0
                .trim_matches(|c: char| !c.is_numeric())
                .split(") (")
                .map(|button_str| {
                    let mut button = vec![false; lights.len()];
                    button_str
                        .split(',')
                        .map(|toggle_str| toggle_str.parse::<usize>().unwrap())
                        .for_each(|toggle| button[toggle] = true);
                    return button;
                })
                .collect();
            let joltages = parts
                .1
                .trim_matches(|c: char| !c.is_numeric())
                .split(',')
                .map(|num_str| num_str.parse().unwrap())
                .collect();
            (lights, buttons, joltages)
        })
        .collect();

    let presses: u64 = machines
        .par_iter()
        .map(|(_, actions, goal)| {
            if goal.iter().all(|joltage| joltage == &0) {
                return 0;
            }
            let mut depth: Option<u64> = None;
            let mut queue: VecDeque<(Vec<u64>, u64)> = VecDeque::new();
            let mut seen: HashSet<Vec<u64>> = HashSet::new();
            queue.push_back((vec![0; goal.len()], 0));
            seen.insert(vec![0; goal.len()]);

            while let Some((current_joltage, current_depth)) = queue.pop_front() {
                for action in actions {
                    let next_joltage: Vec<u64> = current_joltage
                        .iter()
                        .zip(action)
                        .map(|(&joltage, &increase)| if increase { joltage + 1 } else { joltage })
                        .collect();
                    if &next_joltage == goal {
                        depth = Some(current_depth + 1);
                        queue.clear();
                        break;
                    } else if !seen.contains(&next_joltage)
                        && !next_joltage
                            .iter()
                            .zip(goal)
                            .any(|(next, goal)| next > goal)
                    {
                        seen.insert(next_joltage.clone());
                        queue.push_back((next_joltage, current_depth + 1));
                    }
                }
            }

            depth.unwrap()
        })
        .sum();
    println!("Presses: {presses}");
}

fn main() {
    print!("[Sample] ");
    solve(include_str!("../../input/input.sample.txt"));
    print!("[Real] ");
    solve(include_str!("../../input/input.real.txt"));
}
