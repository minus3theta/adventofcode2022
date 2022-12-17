use ezio::stdio;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

const MAX_COORD: i32 = 4000000;
const TUNE: i64 = 4000000;

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
    let Coord(x, y) = (0..=MAX_COORD)
        .find_map(|line_y| scan(&input, line_y))
        .unwrap();
    println!("{}", x as i64 * TUNE + y as i64);

    Ok(())
}

fn scan(input: &[(Coord, Coord)], line_y: i32) -> Option<Coord> {
    let mut ranges = input
        .iter()
        .map(|&(s, b)| x_range(s, b, line_y))
        .filter(|&(l, r)| l != r)
        .collect_vec();
    ranges.sort();
    let mut right = i32::MIN;
    for &(l, r) in &ranges {
        if l > right && (0..=MAX_COORD).contains(&right) {
            return Some(Coord(right, line_y));
        }
        right = right.max(r);
    }
    None
}

fn x_range(sensor: Coord, beacon: Coord, line_y: i32) -> (i32, i32) {
    let dist = sensor.dist(beacon);
    let rest = dist - (line_y - sensor.1).abs();
    if rest < 0 {
        (0, 0)
    } else {
        let l = sensor.0 - rest;
        let r = sensor.0 + rest;
        (l, r + 1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coord(i32, i32);

impl Coord {
    fn dist(&self, other: Self) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}
