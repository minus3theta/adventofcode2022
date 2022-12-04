use std::collections::BTreeSet;

use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut ans = 0;
    for l in &stdio::stdin().into_iter().chunks(3) {
        let mut es = l
            .into_iter()
            .map(|e| e.chars().map(priority).collect::<BTreeSet<_>>());
        let mut common = es.next().unwrap();
        for _ in 0..2 {
            common = &common & &es.next().unwrap();
        }
        ans += common.into_iter().next().unwrap();
    }
    println!("{}", ans);

    Ok(())
}

fn priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => unreachable!(),
    }
}
