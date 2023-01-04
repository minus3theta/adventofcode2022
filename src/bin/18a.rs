use std::collections::HashSet;

use ezio::stdio;
use itertools::Itertools;

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
    let mut ans = 0;
    for &(x, y, z) in &blocks {
        for (dx, dy, dz) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            if !blocks.contains(&(x + dx, y + dy, z + dz)) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);

    Ok(())
}
