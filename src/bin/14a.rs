use std::collections::{BTreeMap, BTreeSet};

use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut field = BTreeMap::<_, BTreeSet<_>>::new();
    for input in stdio::stdin() {
        let coords = input.split(" -> ").map(|xy| {
            let (x, y) = xy.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        });
        for (p, q) in coords.tuple_windows() {
            if p.0 == q.0 {
                let x = p.0;
                for y in p.1.min(q.1)..=p.1.max(q.1) {
                    fill(&mut field, (x, y));
                }
            } else {
                assert_eq!(p.1, q.1);
                let y = p.1;
                for x in p.0.min(q.0)..=p.0.max(q.0) {
                    fill(&mut field, (x, y));
                }
            }
        }
    }
    println!("{}", solve(&mut field));

    Ok(())
}

fn solve(field: &mut BTreeMap<i32, BTreeSet<i32>>) -> i32 {
    let pour = (500, 0);
    for i in 0.. {
        let mut p = pour;
        loop {
            if let Some(y) = drop(field, p) {
                p.1 = y;
                if let Some(q) = drift(field, p) {
                    p = q;
                } else {
                    fill(field, p);
                    break;
                }
            } else {
                return i;
            }
        }
    }
    unreachable!()
}

fn drop(field: &BTreeMap<i32, BTreeSet<i32>>, start: (i32, i32)) -> Option<i32> {
    assert!(!occupied(field, start));
    Some(field.get(&start.0)?.range(start.1..).next()? - 1)
}

fn drift(field: &BTreeMap<i32, BTreeSet<i32>>, start: (i32, i32)) -> Option<(i32, i32)> {
    let (x, y) = start;
    assert!(occupied(field, (x, y + 1)));
    if !occupied(field, (x - 1, y + 1)) {
        Some((x - 1, y + 1))
    } else if !occupied(field, (x + 1, y + 1)) {
        Some((x + 1, y + 1))
    } else {
        None
    }
}

fn fill(field: &mut BTreeMap<i32, BTreeSet<i32>>, p: (i32, i32)) {
    field.entry(p.0).or_default().insert(p.1);
}

fn occupied(field: &BTreeMap<i32, BTreeSet<i32>>, p: (i32, i32)) -> bool {
    field.get(&p.0).map_or(false, |col| col.contains(&p.1))
}
