use std::collections::HashSet;

use anyhow::{bail, Context};
use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut trace = HashSet::new();
    let mut head = (0i32, 0i32);
    let mut tail = (0, 0);
    trace.insert(tail);
    for input in stdio::stdin() {
        let (dir, step) = input
            .split_ascii_whitespace()
            .collect_tuple()
            .context("invalid move")?;
        let step: i32 = step.parse()?;
        for _ in 0..step {
            match dir {
                "R" => head.0 += 1,
                "U" => head.1 += 1,
                "L" => head.0 -= 1,
                "D" => head.1 -= 1,
                _ => bail!("invalid direction"),
            }
            if (head.0 - tail.0).abs() >= 2 {
                tail.1 = head.1;
                if head.0 > tail.0 {
                    tail.0 += 1;
                } else {
                    tail.0 -= 1;
                }
            } else if (head.1 - tail.1).abs() >= 2 {
                tail.0 = head.0;
                if head.1 > tail.1 {
                    tail.1 += 1;
                } else {
                    tail.1 -= 1;
                }
            }
            trace.insert(tail);
        }
    }

    println!("{}", trace.len());

    Ok(())
}
