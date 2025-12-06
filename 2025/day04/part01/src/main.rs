use rayon::prelude::*;

fn solve(input: String) {
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();
    let accessible_rolls: usize = map
        .par_iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, val)| {
                    if !**val {
                        return false;
                    }

                    let mut count = 0;
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx != 0 || dy != 0 {
                                let x: i32 = *x as i32 + dx;
                                let y: i32 = y as i32 + dy;
                                if x >= 0
                                    && x < row.len() as i32
                                    && y >= 0
                                    && y < map.len() as i32
                                    && map[y as usize][x as usize]
                                {
                                    count += 1;
                                }
                            }
                        }
                    }

                    return count < 4;
                })
                .count()
        })
        .sum();

    println!("Accessible rolls: {accessible_rolls}");
}

fn main() {
    let input_sample = String::from(include_str!("../../input/input.sample.txt"));
    let input_real = String::from(include_str!("../../input/input.real.txt"));

    print!("[Sample] ");
    solve(input_sample);
    print!("[Real] ");
    solve(input_real);
}
