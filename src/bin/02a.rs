use ezio::stdio;

fn main() -> anyhow::Result<()> {
    let mut hands = vec![];
    for l in stdio::stdin() {
        let mut l = l.chars();
        let o = l.next().unwrap();
        let p = l.last().unwrap();
        hands.push((o, p));
    }

    let ans = hands
        .iter()
        .map(|&(o, p)| hand(p) + outcome(o, p))
        .sum::<i32>();

    println!("{}", ans);

    Ok(())
}

fn hand(h: char) -> i32 {
    match h {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => unreachable!(),
    }
}

fn outcome(o: char, p: char) -> i32 {
    match (o, p) {
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        _ => 0,
    }
}
