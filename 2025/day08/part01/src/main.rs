use rayon::prelude::*;

fn calculate_distance(this_edge: (u64, u64, u64), that_edge: (u64, u64, u64)) -> f64 {
    let (d0, d1, d2): (f64, f64, f64) = (
        this_edge.0 as f64 - that_edge.0 as f64,
        this_edge.1 as f64 - that_edge.1 as f64,
        this_edge.2 as f64 - that_edge.2 as f64,
    );
    return (d0 * d0 + d1 * d1 + d2 * d2).sqrt();
}

fn solve(input: &str, vertice_count: usize) {
    let vertices: Vec<(u64, u64, u64)> = input
        .lines()
        .map(|line| {
            let (x, part) = line.split_once(',').unwrap();
            let (y, z) = part.split_once(',').unwrap();
            return (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
        })
        .collect();

    let mut distances: Vec<Vec<f64>> = (0..vertices.len())
        .into_par_iter()
        .map(|i| {
            (0..vertices.len())
                .map(|j| {
                    if j <= i {
                        0.0
                    } else {
                        calculate_distance(vertices[i], vertices[j])
                    }
                })
                .collect()
        })
        .collect();
    for i in 0..vertices.len() {
        for j in 0..i {
            distances[i][j] = distances[j][i];
        }
    }

    let mut edges: Vec<(f64, usize, usize)> =
        Vec::with_capacity(vertices.len() * (vertices.len() - 1) / 2);
    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            edges.push((distances[i][j], i, j));
        }
    }
    edges.sort_unstable_by(|vertice1, vertice2| f64::total_cmp(&vertice1.0, &vertice2.0));
    edges.truncate(vertice_count);

    let mut circuits: Vec<Vec<bool>> = Vec::new();
    for (_, vertex1, vertex2) in edges {
        let (mut maybe_found1, mut maybe_found2) = (None, None);
        for (i, circuit) in (&mut circuits).iter().enumerate() {
            if maybe_found1.is_none() && circuit[vertex1] {
                maybe_found1 = Some(i);
            }
            if maybe_found2.is_none() && circuit[vertex2] {
                maybe_found2 = Some(i);
            }
        }
        if let Some(found1) = maybe_found1 {
            if let Some(found2) = maybe_found2 {
                if found1 != found2 {
                    let (keep, remove) = if found1 < found2 {
                        (found1, found2)
                    } else {
                        (found2, found1)
                    };
                    let removed = circuits.remove(remove);
                    for (i, entry) in removed.into_iter().enumerate() {
                        if entry {
                            circuits[keep][i] = true;
                        }
                    }
                }
            } else {
                circuits[found1][vertex2] = true;
            }
        } else if let Some(found2) = maybe_found2 {
            circuits[found2][vertex1] = true;
        } else {
            circuits.push(vec![false; vertices.len()]);
            circuits.last_mut().unwrap()[vertex1] = true;
            circuits.last_mut().unwrap()[vertex2] = true;
        }
    }

    let mut circuit_counts: Vec<usize> = circuits
        .iter()
        .map(|circuit| circuit.iter().filter(|vertex| **vertex).count())
        .collect();
    circuit_counts.sort_unstable_by(|a, b| b.cmp(a));
    let res: usize = circuit_counts.iter().take(3).product();

    println!("The big three: {res}");
}

fn main() {
    print!("[Sample] ");
    solve(include_str!("../../input/input.sample.txt"), 10);
    print!("[Real] ");
    solve(include_str!("../../input/input.real.txt"), 1000);
}
