use std::{iter::Peekable, num::ParseIntError, str::FromStr};

use anyhow::{bail, Context};
use ezio::stdio;
use itertools::{Either, Itertools};

fn main() -> anyhow::Result<()> {
    let mut ans = 0;
    for (i, input) in stdio::stdin().into_iter().chunks(3).into_iter().enumerate() {
        let input = input
            .take(2)
            .map(|s| s.parse())
            .collect::<Result<Vec<Packet>, _>>()?;
        if input[0] < input[1] {
            ans += i + 1;
        }
    }
    println!("{}", ans);

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = Token::tokenize(s)?.into_iter().peekable();
        Self::from_tokens(&mut tokens)
    }
}

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.partial_cmp(r),
            (&Packet::Int(l), Packet::List(r)) => vec![Packet::Int(l)].partial_cmp(r),
            (Packet::List(l), &Packet::Int(r)) => l.partial_cmp(&vec![Packet::Int(r)]),
            (Packet::List(l), Packet::List(r)) => l.partial_cmp(r),
        }
    }
}

impl Packet {
    fn from_tokens(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> anyhow::Result<Self> {
        match tokens.next().context("empty")? {
            Token::L => {
                let mut packets = vec![];
                loop {
                    if let Some(&Token::R) = tokens.peek() {
                        tokens.next();
                        break;
                    }
                    packets.push(Self::from_tokens(tokens)?);
                }
                Ok(Packet::List(packets))
            }
            Token::R => bail!("unexpected right bracket"),
            Token::Int(v) => Ok(Packet::Int(v)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    L,
    R,
    Int(i32),
}

impl Token {
    fn tokenize(input: &str) -> Result<Vec<Self>, ParseIntError> {
        input
            .split_inclusive(&['[', ']'])
            .flat_map(|s| s.split(','))
            .flat_map(|s| match s.split_once(']') {
                None => Either::Left([s].into_iter()),
                Some((l, _)) => Either::Right([l, "]"].into_iter()),
            })
            .filter_map(|tok| match tok {
                "[" => Some(Ok(Token::L)),
                "]" => Some(Ok(Token::R)),
                "" => None,
                _ => Some(tok.parse().map(Token::Int)),
            })
            .collect()
    }
}
