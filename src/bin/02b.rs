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
        .map(|&(o, p)| hand(o, p) + outcome(p))
        .sum::<i32>();

    println!("{}", ans);

    Ok(())
}

fn hand(o: char, p: char) -> i32 {
    match (o, p) {
        ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1,
        ('B', 'Y') | ('C', 'X') | ('A', 'Z') => 2,
        ('C', 'Y') | ('A', 'X') | ('B', 'Z') => 3,
        _ => unreachable!(),
    }
}

fn outcome(p: char) -> i32 {
    match p {
        'Z' => 6,
        'Y' => 3,
        _ => 0,
    }
}
