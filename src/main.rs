#![feature(slice_group_by)]

mod matcher;
use anyhow::Result;
use clap::{command, Parser};
use matcher::BenchResult;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    input: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let m = matcher::Matcher::new();

    let mut results = vec![];

    if !cli.input.is_empty() {
        for path in cli.input {
            let mut f = BufReader::new(File::open(path)?);
            let mut res = m.process(&mut f)?;
            results.append(&mut res);
        }
    } else {
        let mut r = BufReader::new(io::stdin());
        let mut res = m.process(&mut r)?;
        results.append(&mut res);
    }

    if results.is_empty() {
        return Ok(());
    }

    let results = group_results(&results);
    dbg!(results);

    Ok(())
}

fn group_results(results: &[BenchResult]) -> HashMap<String, Vec<&BenchResult>> {
    let mut res = HashMap::new();

    for r in results {
        let entry = res.entry(r.name.clone()).or_insert(vec![]);
        entry.push(r);
    }

    res
}
