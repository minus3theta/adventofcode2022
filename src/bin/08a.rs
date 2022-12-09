use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = stdio::stdin()
        .into_iter()
        .map(|s| s.chars().map(|c| c as i8 - b'0' as i8).collect_vec())
        .collect_vec();
    let h = input.len();
    let w = input[0].len();
    let mut visible = vec![vec![false; w]; h];
    for x in 0..w {
        let mut max_height = -1;
        for y in 0..h {
            if input[y][x] > max_height {
                max_height = input[y][x];
                visible[y][x] = true;
            }
        }

        let mut max_height = -1;
        for y in (0..h).rev() {
            if input[y][x] > max_height {
                max_height = input[y][x];
                visible[y][x] = true;
            }
        }
    }

    for y in 0..h {
        let mut max_height = -1;
        for x in 0..w {
            if input[y][x] > max_height {
                max_height = input[y][x];
                visible[y][x] = true;
            }
        }

        let mut max_height = -1;
        for x in (0..w).rev() {
            if input[y][x] > max_height {
                max_height = input[y][x];
                visible[y][x] = true;
            }
        }
    }

    println!(
        "{}",
        visible
            .iter()
            .map(|r| r.iter().filter(|&&v| v).count())
            .sum::<usize>()
    );

    Ok(())
}
