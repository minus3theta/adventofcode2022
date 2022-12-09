use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = stdio::stdin()
        .into_iter()
        .map(|s| s.chars().map(|c| c as i8 - b'0' as i8).collect_vec())
        .collect_vec();
    let h = input.len();
    let w = input[0].len();
    let count = |mut x: usize, mut y: usize, dx: usize, dy: usize| {
        let center = input[y][x];
        let mut s = 0;
        loop {
            x = x.wrapping_add(dx);
            y = y.wrapping_add(dy);
            if x >= w || y >= h {
                break;
            }
            s += 1;
            if input[y][x] >= center {
                break;
            }
        }
        s
    };
    let score = |x, y| {
        count(x, y, 0, 1)
            * count(x, y, 1, 0)
            * count(x, y, 0, usize::MAX)
            * count(x, y, usize::MAX, 0)
    };

    let ans = (0..w)
        .cartesian_product(0..h)
        .map(|(x, y)| score(x, y))
        .max()
        .unwrap();

    println!("{}", ans);

    Ok(())
}
