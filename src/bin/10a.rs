use anyhow::Context;
use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut cycle = 0;
    let mut x = 1;
    let mut ans = 0;
    let mut strength = 20;
    let mut put = |clock, x| {
        cycle += clock;
        if cycle >= strength {
            ans += strength * x;
            strength += 40;
        }
    };

    for input in stdio::stdin() {
        match input.as_str() {
            "noop" => {
                put(1, x);
            }
            _ => {
                let (_, v) = input
                    .split_ascii_whitespace()
                    .collect_tuple()
                    .context("invalid operation")?;
                let v: i32 = v.parse()?;
                put(2, x);
                x += v;
            }
        }
    }
    println!("{}", ans);

    Ok(())
}
