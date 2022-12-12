use anyhow::Context;
use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut cycle = 1;
    let mut x = 1;
    let mut put = |x: i32| {
        if (x - cycle + 1).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        cycle += 1;
        if cycle > 40 {
            cycle = 1;
            println!();
        }
    };

    for input in stdio::stdin() {
        match input.as_str() {
            "noop" => {
                put(x);
            }
            _ => {
                let (_, v) = input
                    .split_ascii_whitespace()
                    .collect_tuple()
                    .context("invalid operation")?;
                let v: i32 = v.parse()?;
                put(x);
                put(x);
                x += v;
            }
        }
    }

    Ok(())
}
