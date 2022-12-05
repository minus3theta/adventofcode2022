use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut init = vec![];
    let mut stack: Vec<Vec<char>> = vec![];
    let mut iter = stdio::stdin().into_iter();
    for l in &mut iter {
        if !l.contains('[') {
            let len = l.split_ascii_whitespace().count();
            stack = vec![vec![]; len];
            break;
        }
        init.push(l);
    }
    for layer in init.iter().rev() {
        for (i, mut block) in layer.chars().chunks(4).into_iter().enumerate() {
            let b = block.nth(1).unwrap();
            if b != ' ' {
                stack[i].push(b);
            }
        }
    }
    iter.next();
    for l in &mut iter {
        let (_, amount, _, from, _, to) = l.split_ascii_whitespace().collect_tuple().unwrap();
        let amount: usize = amount.parse()?;
        let from = from.parse::<usize>()? - 1;
        let to = to.parse::<usize>()? - 1;
        for _ in 0..amount {
            let b = stack[from].pop().unwrap();
            stack[to].push(b);
        }
    }
    for col in &stack {
        print!("{}", col.last().unwrap());
    }
    println!();

    Ok(())
}
