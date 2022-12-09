use std::{collections::BTreeMap, iter::Peekable, str::FromStr};

use anyhow::{bail, Context};
use ezio::stdio;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut input = stdio::stdin().into_iter().peekable();

    let mut root = Directory::default();
    while input.peek().is_some() {
        root.process(&mut input)?;
    }

    println!("{}", solve(&root).1);

    Ok(())
}

const UPPER_BOUND: i32 = 100000;

fn solve(dir: &Directory) -> (i32, i32) {
    let mut current_size = 0;
    let mut current_ans = 0;
    for sub in dir.subdir.values() {
        let (size, ans) = solve(sub);
        current_size += size;
        current_ans += ans;
    }
    for file in dir.file.values() {
        current_size += file.size;
    }
    if current_size <= UPPER_BOUND {
        current_ans += current_size;
    }
    (current_size, current_ans)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Command {
    Cd(String),
    Ls,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "$ ls" => Ok(Command::Ls),
            s if s.starts_with("$ cd ") => {
                Ok(Command::Cd(s.trim_start_matches("$ cd ").to_owned()))
            }
            _ => bail!("not a command"),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Directory {
    subdir: BTreeMap<String, Directory>,
    file: BTreeMap<String, File>,
}

impl Directory {
    fn process(
        &mut self,
        input: &mut Peekable<impl Iterator<Item = String>>,
    ) -> anyhow::Result<bool> {
        while let Some(command) = input.next() {
            let command = command.parse()?;
            match command {
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        return Ok(true);
                    }
                    ".." => {
                        return Ok(false);
                    }
                    _ => {
                        let current = self.subdir.entry(path).or_insert_with(Default::default);
                        if current.process(input)? {
                            return Ok(true);
                        }
                    }
                },
                Command::Ls => {
                    while let Some(line) = input.peek() {
                        if line.starts_with("$ ") {
                            break;
                        }
                        let record = input.next().unwrap();
                        if record.starts_with("dir") {
                            continue;
                        }
                        let (size, name) = record
                            .split_ascii_whitespace()
                            .collect_tuple()
                            .context("failed to parse record")?;
                        self.file.insert(name.to_owned(), File::new(size.parse()?));
                    }
                }
            }
        }
        Ok(false)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct File {
    size: i32,
}

impl File {
    fn new(size: i32) -> Self {
        Self { size }
    }
}
