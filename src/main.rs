#![feature(slice_group_by)]

mod matcher;
use anyhow::Result;
use clap::{command, Parser};
use conv::{ConvUtil, ValueFrom};
use matcher::BenchResult;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    iter::Sum,
    ops::{Add, Div},
    path::PathBuf,
    slice::Iter,
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

    for (name, results) in results.iter() {
        let s = stats(results.iter().map(|v| v.ops.unwrap()));
        dbg!(s);
    }

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

#[derive(Debug)]
struct Stats {
    pub n: usize,
    pub min: usize,
    pub max: usize,
    pub sum: usize,
    pub avg: f64,
    pub sd: f64,
    pub median: usize,
    pub pct90: usize,
    pub pct95: usize,
    pub pct99: usize,
}

fn stats<I>(iter: I) -> Option<Stats>
where
    I: Iterator<Item = usize> + Clone,
{
    let n = iter.clone().count();
    let min = iter.clone().min()?;
    let max = iter.clone().max()?;
    let sum: usize = iter.clone().sum();
    let avg = sum.value_as::<f64>().unwrap() / n as f64;
    let sd = (iter
        .clone()
        .map(|v| v.value_as::<f64>().unwrap())
        .fold(0f64, |acc, v| acc + (v - avg).powf(2f64))
        / n as f64)
        .sqrt();

    let median = if n % 2 == 1 {
        iter.clone().nth((n + 1) / 2 - 1)?
    } else {
        let l = iter.clone().nth(n / 2 - 1)?;
        let r = iter.clone().nth(n / 2)?;
        (l + r) / 2usize
    };

    let pct90 = get_nth_percentile(n, iter.clone(), 0.90)?;
    let pct95 = get_nth_percentile(n, iter.clone(), 0.95)?;
    let pct99 = get_nth_percentile(n, iter.clone(), 0.99)?;

    Some(Stats {
        n,
        min,
        max,
        sum,
        avg,
        sd,
        median,
        pct90,
        pct95,
        pct99,
    })
}

fn get_nth_percentile<I>(count: usize, times: I, percentile: f64) -> Option<usize>
where
    I: Iterator<Item = usize> + Clone,
{
    let el = count as f64 * percentile;
    let el_trunc = el as isize - 1;
    if el_trunc < 0 {
        return times.clone().next();
    }

    if el_trunc as usize + 1 >= count {
        return times.clone().nth(el_trunc as usize);
    }

    let el_a = times.clone().nth(el_trunc as usize)?;
    let el_b = times.clone().nth(el_trunc as usize + 1)?;

    let el_fract_b = el - el_trunc as f64;
    let el_fract_a = 1f64 - el_fract_b;

    let res = (el_a as f64 * el_fract_a + el_b as f64 * el_fract_b).round();

    Some(res as usize)
}
