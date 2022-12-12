use std::str::FromStr;

use anyhow::{bail, Context};
use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = stdio::stdin().into_iter().join("\n");
    let input = input
        .split("\n\n")
        .map(|s| s.parse())
        .collect::<Result<Vec<State>, _>>()?;
    let (mut state, monkey): (Vec<Items>, Vec<Monkey>) =
        input.into_iter().map(|State(s, m)| (s, m)).unzip();
    let mut count = vec![0; monkey.len()];
    for _ in 0..20 {
        for (i, m) in monkey.iter().enumerate() {
            let mut items = vec![];
            std::mem::swap(&mut items, &mut state[i]);
            count[i] += items.len();
            m.process(items, &mut state);
        }
    }
    count.sort();
    let ans = count.pop().unwrap() * count.pop().unwrap();
    println!("{}", ans);

    Ok(())
}

#[derive(Debug)]
enum Operation {
    Add(i64),
    Mult(i64),
    Square,
}

impl Operation {
    fn apply(&self, old: i64) -> i64 {
        match self {
            Operation::Add(r) => old + r,
            Operation::Mult(r) => old * r,
            Operation::Square => old * old,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    operation: Operation,
    divisor: i64,
    throw_true: usize,
    throw_false: usize,
}

impl Monkey {
    fn process(&self, items: Items, state: &mut [Items]) {
        for item in items {
            let value = self.operation.apply(item) / 3;
            let destination = if value % self.divisor == 0 {
                self.throw_true
            } else {
                self.throw_false
            };
            state[destination].push(value);
        }
    }
}

type Items = Vec<i64>;

#[derive(Debug)]
struct State(Items, Monkey);

impl FromStr for State {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        iter.next();
        let (_, items) = iter
            .next()
            .context("no items")?
            .split(": ")
            .collect_tuple()
            .context("invalid items")?;
        let items = items
            .split(", ")
            .map(FromStr::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let (_, operation) = iter
            .next()
            .context("no operation")?
            .split("= old ")
            .collect_tuple()
            .context("invalid operation")?;
        let operation = match operation
            .split_ascii_whitespace()
            .collect_tuple()
            .context("invalid operation")?
        {
            ("*", "old") => Operation::Square,
            ("*", rhs) => Operation::Mult(rhs.parse()?),
            ("+", rhs) => Operation::Add(rhs.parse()?),
            _ => bail!("invalid operation"),
        };
        let divisor = iter
            .next()
            .context("no test")?
            .split_ascii_whitespace()
            .last()
            .context("no test")?
            .parse()?;
        let throw_true = iter
            .next()
            .context("no throw_true")?
            .split_ascii_whitespace()
            .last()
            .context("no throw_true")?
            .parse()?;
        let throw_false = iter
            .next()
            .context("no throw_false")?
            .split_ascii_whitespace()
            .last()
            .context("no throw_false")?
            .parse()?;

        Ok(Self(
            items,
            Monkey {
                operation,
                divisor,
                throw_true,
                throw_false,
            },
        ))
    }
}
