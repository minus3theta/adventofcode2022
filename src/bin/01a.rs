use ezio::stdio;

fn main() -> anyhow::Result<()> {
    let mut elves = vec![];
    let mut current = vec![];
    for l in stdio::stdin() {
        if l.is_empty() {
            elves.push(current);
            current = vec![];
        } else {
            let x = l.parse::<i32>()?;
            current.push(x);
        }
    }
    elves.push(current);

    let ans = elves.iter().map(|e| e.iter().sum::<i32>()).max().unwrap();
    println!("{}", ans);

    Ok(())
}
