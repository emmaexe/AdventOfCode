use itertools::Itertools;
use rayon::prelude::*;

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
        .map(|(goal, actions, _)| {
            if goal.iter().all(|bit| !bit) {
                return 0;
            }

            (1..goal.len())
                .into_par_iter()
                .by_exponential_blocks()
                .find_first(|depth| {
                    actions
                        .iter()
                        .combinations(*depth)
                        .par_bridge()
                        .any(|buttons| {
                            buttons.iter().fold(vec![false; goal.len()], |mut acc, b| {
                                for (i, &bit) in b.iter().enumerate() {
                                    acc[i] ^= bit;
                                }
                                acc
                            }) == *goal
                        })
                })
                .unwrap_or(actions.len()) as u64
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
