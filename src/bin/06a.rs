use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = stdio::read_line();
    let input = input.chars().collect_vec();
    for (pos, win) in input.windows(4).enumerate() {
        if win.iter().all_unique() {
            println!("{}", pos + 4);
            break;
        }
    }

    Ok(())
}
