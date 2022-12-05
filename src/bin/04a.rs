use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut ans = 0;
    for l in stdio::stdin() {
        let (x, y) = l
            .split(',')
            .map(|r| {
                r.split('-')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect_tuple::<(_, _)>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();
        if includes(x, y) || includes(y, x) {
            ans += 1
        }
    }
    println!("{}", ans);

    Ok(())
}

fn includes(x: (i32, i32), y: (i32, i32)) -> bool {
    x.0 <= y.0 && y.1 <= x.1
}
