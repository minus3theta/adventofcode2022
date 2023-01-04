use std::collections::{HashSet, VecDeque};

use ezio::stdio;
use itertools::Itertools;

const ADJ: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn extend(range: (i32, i32)) -> (i32, i32) {
    (range.0 - 1, range.1 + 1)
}

fn main() -> anyhow::Result<()> {
    let mut blocks = HashSet::<(i32, i32, i32)>::new();
    for l in stdio::stdin() {
        blocks.insert(
            l.split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap(),
        );
    }
    let x = blocks.iter().map(|&(x, _, _)| x).minmax();
    let y = blocks.iter().map(|&(_, y, _)| y).minmax();
    let z = blocks.iter().map(|&(_, _, z)| z).minmax();
    let (x0, x1) = extend(x.into_option().unwrap());
    let (y0, y1) = extend(y.into_option().unwrap());
    let (z0, z1) = extend(z.into_option().unwrap());
    let origin = (x0, y0, z0);
    assert!(!blocks.contains(&origin));
    let mut visited = HashSet::new();
    visited.insert(origin);
    let mut que = VecDeque::new();
    que.push_back(origin);
    while let Some((x, y, z)) = que.pop_front() {
        for &(dx, dy, dz) in &ADJ {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if (x0..=x1).contains(&nx)
                && (y0..=y1).contains(&ny)
                && (z0..=z1).contains(&nz)
                && !visited.contains(&(nx, ny, nz))
                && !blocks.contains(&(nx, ny, nz))
            {
                visited.insert((nx, ny, nz));
                que.push_back((nx, ny, nz));
            }
        }
    }
    let mut ans = 0;
    for &(x, y, z) in &blocks {
        for &(dx, dy, dz) in &ADJ {
            if visited.contains(&(x + dx, y + dy, z + dz)) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);

    Ok(())
}
