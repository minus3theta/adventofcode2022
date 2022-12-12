use std::collections::HashSet;

use anyhow::{bail, Context};
use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut trace = HashSet::new();
    let mut chain = vec![(0i32, 0i32); 10];
    trace.insert(chain[9]);
    for input in stdio::stdin() {
        let (dir, step) = input
            .split_ascii_whitespace()
            .collect_tuple()
            .context("invalid move")?;
        let step: i32 = step.parse()?;
        for _ in 0..step {
            match dir {
                "R" => chain[0].0 += 1,
                "U" => chain[0].1 += 1,
                "L" => chain[0].0 -= 1,
                "D" => chain[0].1 -= 1,
                _ => bail!("invalid direction"),
            }
            for i in 0..9 {
                chain[i + 1] = follow(chain[i], chain[i + 1]);
            }
            trace.insert(chain[9]);
        }
    }

    println!("{}", trace.len());

    Ok(())
}

fn follow(head: (i32, i32), mut tail: (i32, i32)) -> (i32, i32) {
    if (head.0 - tail.0).abs().max((head.1 - tail.1).abs()) >= 2 {
        use std::cmp::Ordering::*;
        match head.0.cmp(&tail.0) {
            Less => tail.0 -= 1,
            Equal => (),
            Greater => tail.0 += 1,
        }
        match head.1.cmp(&tail.1) {
            Less => tail.1 -= 1,
            Equal => (),
            Greater => tail.1 += 1,
        }
    }
    tail
}
