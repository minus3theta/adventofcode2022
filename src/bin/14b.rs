use std::collections::{BTreeMap, BTreeSet};

use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut field = BTreeMap::<_, BTreeSet<_>>::new();
    let mut bottom = 0;
    for input in stdio::stdin() {
        let coords = input.split(" -> ").map(|xy| {
            let (x, y) = xy.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        });
        for (p, q) in coords.tuple_windows() {
            bottom = bottom.max(p.1.max(q.1) + 2);
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
    println!("{}", solve(&mut field, bottom));

    Ok(())
}

fn solve(field: &mut BTreeMap<i32, BTreeSet<i32>>, bottom: i32) -> i32 {
    let pour = (500, 0);
    for i in 0.. {
        let mut p = pour;
        loop {
            if occupied(field, bottom, p) {
                return i;
            }

            p.1 = drop(field, bottom, p);
            if let Some(q) = drift(field, bottom, p) {
                p = q;
            } else {
                fill(field, p);
                break;
            }
        }
    }
    unreachable!()
}

fn drop(field: &BTreeMap<i32, BTreeSet<i32>>, bottom: i32, start: (i32, i32)) -> i32 {
    field
        .get(&start.0)
        .and_then(|f| f.range(start.1..).next())
        .unwrap_or(&bottom)
        - 1
}

fn drift(
    field: &BTreeMap<i32, BTreeSet<i32>>,
    bottom: i32,
    start: (i32, i32),
) -> Option<(i32, i32)> {
    let (x, y) = start;
    assert!(occupied(field, bottom, (x, y + 1)));
    if !occupied(field, bottom, (x - 1, y + 1)) {
        Some((x - 1, y + 1))
    } else if !occupied(field, bottom, (x + 1, y + 1)) {
        Some((x + 1, y + 1))
    } else {
        None
    }
}

fn fill(field: &mut BTreeMap<i32, BTreeSet<i32>>, p: (i32, i32)) {
    field.entry(p.0).or_default().insert(p.1);
}

fn occupied(field: &BTreeMap<i32, BTreeSet<i32>>, bottom: i32, p: (i32, i32)) -> bool {
    if p.1 == bottom {
        return true;
    }
    field.get(&p.0).map_or(false, |col| col.contains(&p.1))
}
