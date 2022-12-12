use std::collections::VecDeque;

use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut start = None;
    let mut goal = None;
    let height = stdio::stdin()
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            row.bytes()
                .enumerate()
                .map(|(j, c)| match c {
                    b'S' => {
                        start = Some((i, j));
                        0
                    }
                    b'E' => {
                        goal = Some((i, j));
                        b'z' - b'a'
                    }
                    b'a'..=b'z' => c - b'a',
                    _ => 0,
                })
                .collect_vec()
        })
        .collect_vec();
    let h = height.len();
    let w = height[0].len();
    let start = start.unwrap();
    let goal = goal.unwrap();
    let mut dist = vec![vec![i32::MAX; w]; h];
    let mut que = VecDeque::new();
    que.push_back(start);
    dist[start.0][start.1] = 0;
    while let Some(p) = que.pop_front() {
        for &(di, dj) in &[(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)] {
            let q = (p.0.wrapping_add(di), p.1.wrapping_add(dj));
            if q.0 < h && q.1 < w && height[q.0][q.1] <= height[p.0][p.1] + 1 {
                let d = dist[p.0][p.1] + 1;
                if d < dist[q.0][q.1] {
                    dist[q.0][q.1] = d;
                    que.push_back(q);
                }
            }
        }
    }

    println!("{}", dist[goal.0][goal.1]);

    Ok(())
}
