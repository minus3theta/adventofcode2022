use std::collections::BTreeSet;

use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut ans = 0;
    for l in stdio::stdin() {
        let r = l.chars().map(priority).collect_vec();
        assert_eq!(r.len() % 2, 0);
        let mid = r.len() / 2;
        let first = r[..mid].iter().cloned().collect::<BTreeSet<_>>();
        let second = r[mid..].iter().cloned().collect();
        let common = *first.intersection(&second).next().unwrap();
        ans += common;
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
