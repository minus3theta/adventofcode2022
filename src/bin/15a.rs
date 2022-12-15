use ezio::stdio;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap()
    });
    let input = stdio::stdin()
        .into_iter()
        .filter_map(|line| {
            let (sx, sy, bx, by) = RE
                .captures(&line)?
                .iter()
                .skip(1)
                .map(|s| s.unwrap().as_str().parse::<i32>().unwrap())
                .collect_tuple::<(_, _, _, _)>()?;
            Some((Coord(sx, sy), Coord(bx, by)))
        })
        .collect_vec();
    let mut ranges = input
        .iter()
        .map(|&(s, b)| x_range(s, b, 2000000))
        .filter(|&(l, r)| l != r)
        .collect_vec();
    ranges.sort();
    let mut ans = 0;
    let mut left = i32::MIN;
    let mut right = i32::MIN;
    for &(l, r) in &ranges {
        if l > right {
            ans += right - left;
            left = l;
        }
        right = right.max(r);
    }
    ans += right - left;
    println!("{}", ans);

    Ok(())
}

fn x_range(sensor: Coord, beacon: Coord, line_y: i32) -> (i32, i32) {
    let dist = sensor.dist(beacon);
    let rest = dist - (line_y - sensor.1).abs();
    if rest < 0 {
        (0, 0)
    } else {
        let l = sensor.0 - rest;
        let r = sensor.0 + rest;
        match (Coord(l, line_y) == beacon, Coord(r, line_y) == beacon) {
            (true, true) => (0, 0),
            (true, false) => (l + 1, r + 1),
            (false, true) => (l, r),
            (false, false) => (l, r + 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coord(i32, i32);

impl Coord {
    fn dist(&self, other: Self) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensor_8_7() {
        let sensor = Coord(8, 7);
        let beacon = Coord(2, 10);
        assert_eq!(sensor.dist(beacon), 9);
        assert_eq!(x_range(sensor, beacon, 0), (6, 11));
    }
}
