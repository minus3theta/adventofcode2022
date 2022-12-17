use std::collections::HashMap;

use ezio::stdio;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

const TIME: usize = 30;

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
    let mut dp = vec![vec![vec![i64::MIN; 1 << positive]; valves.len()]; TIME + 1];
    dp[0][index["AA"]][0] = 0;
    for t in 0..TIME {
        for (vi, &(flow, ref tunnels)) in valves.iter().enumerate() {
            for opened in 0..1 << positive {
                if flow > 0 && (opened >> vi) & 1 == 0 {
                    chmax(
                        dp[t][vi][opened] + flow * (TIME - t - 1) as i64,
                        &mut dp[t + 1][vi][opened | (1 << vi)],
                    );
                }
                for &next in tunnels {
                    chmax(dp[t][vi][opened], &mut dp[t + 1][next][opened]);
                }
            }
        }
    }
    println!("{}", dp[TIME].iter().flat_map(|v| v.iter()).max().unwrap());

    Ok(())
}

fn chmax(x: i64, y: &mut i64) {
    *y = (*y).max(x);
}
