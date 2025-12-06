use rayon::prelude::*;

fn solve(input: String) {
    let mut map: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    let mut accessible_rolls: usize = 0;
    let mut last_access: usize = 1;
    while last_access != 0 {
        let (next_map, changes): (Vec<Vec<bool>>, Vec<usize>) = map
            .par_iter()
            .enumerate()
            .map(|(y, row)| {
                let mut next_row = row.clone();
                let mut local_changes = 0;

                for (x, val) in row.iter().enumerate() {
                    if !val {
                        continue;
                    }

                    let mut count = 0;
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx != 0 || dy != 0 {
                                let x: i32 = x as i32 + dx;
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

                    if count < 4 {
                        next_row[x] = false;
                        local_changes += 1;
                    }
                }

                (next_row, local_changes)
            })
            .unzip();

        last_access = changes.iter().sum();
        map = next_map;
        accessible_rolls += last_access;
    }

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
