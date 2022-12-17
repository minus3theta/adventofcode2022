use std::collections::HashMap;

use ezio::stdio;
use indicatif::ProgressIterator;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

const TIME: usize = 26;

fn main() -> anyhow::Result<()> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
            .unwrap()
    });
    let mut valves = stdio::stdin()
        .into_iter()
        .map(|line| {
            let (_, valve, flow, tunnels) = RE
                .captures(&line)
                .unwrap()
                .iter()
                .map(|c| c.unwrap().as_str())
                .collect_tuple()
                .unwrap();
            let tunnels = tunnels.split(", ").map(|s| s.to_owned()).collect_vec();
            (valve.to_owned(), flow.parse::<i64>().unwrap(), tunnels)
        })
        .collect_vec();
    valves.sort_by_key(|&(_, flow, _)| std::cmp::Reverse(flow));
    let index = valves
        .iter()
        .enumerate()
        .map(|(i, (v, _, _))| (v.clone(), i))
        .collect::<HashMap<_, _>>();
    let valves = valves
        .into_iter()
        .map(|(_, flow, tunnels)| (flow, tunnels.into_iter().map(|t| index[&t]).collect_vec()))
        .collect_vec();
    let positive = valves.iter().filter(|&&(rate, _)| (rate > 0)).count();
    let mut dp =
        vec![vec![vec![vec![i64::MIN; 1 << positive]; valves.len()]; valves.len()]; TIME + 1];
    let start = index["AA"];
    dp[0][start][start][0] = 0;
    for t in (0..TIME).progress() {
        for v in 0..valves.len() {
            for u in 0..valves.len() {
                for opened in 0..1 << positive {
                    for move_v in moves(v, opened, true, &valves) {
                        let include_open = v != u;
                        for move_u in moves(u, opened, include_open, &valves) {
                            let mut current = dp[t][v][u][opened];
                            let mut next_opened = opened;
                            let next_v = match move_v {
                                Move::Open(flow) => {
                                    current += flow * (TIME - t - 1) as i64;
                                    next_opened |= 1 << v;
                                    v
                                }
                                Move::Go(next_v) => next_v,
                            };
                            let next_u = match move_u {
                                Move::Open(flow) => {
                                    current += flow * (TIME - t - 1) as i64;
                                    next_opened |= 1 << u;
                                    u
                                }
                                Move::Go(next_u) => next_u,
                            };
                            chmax(current, &mut dp[t + 1][next_v][next_u][next_opened]);
                        }
                    }
                }
            }
        }
    }
    println!(
        "{}",
        dp[TIME]
            .iter()
            .flat_map(|v| v.iter().flat_map(|u| u.iter()))
            .max()
            .unwrap()
    );

    Ok(())
}

enum Move {
    Open(i64),
    Go(usize),
}

fn moves(
    valve: usize,
    opened: usize,
    include_open: bool,
    valves: &[(i64, Vec<usize>)],
) -> Vec<Move> {
    let mut ret = valves[valve]
        .1
        .iter()
        .map(|&next| Move::Go(next))
        .collect_vec();
    if include_open && valves[valve].0 > 0 && (opened >> valve) & 1 == 0 {
        ret.push(Move::Open(valves[valve].0));
    }

    ret
}

fn chmax(x: i64, y: &mut i64) {
    *y = (*y).max(x);
}
