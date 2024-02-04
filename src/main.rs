#![feature(slice_group_by)]

mod analysis;
mod matcher;

use crate::analysis::Stats;
use anyhow::Result;
use clap::{command, Parser};
use matcher::BenchResult;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};
use tabled::{settings::Style, Table, Tabled};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    input: Vec<PathBuf>,
}

#[derive(Tabled)]
struct StatsEntry {
    metric: String,
    min: usize,
    max: usize,
    avg: f64,
    sd: f64,
    median: usize,
    pct90: usize,
    pct95: usize,
    pct99: usize,
}

impl StatsEntry {
    fn from_stats(metric: &str, stats: &Stats) -> StatsEntry {
        StatsEntry {
            metric: metric.to_string(),
            min: stats.min,
            max: stats.max,
            avg: stats.avg,
            sd: stats.sd,
            median: stats.median,
            pct90: stats.pct90,
            pct95: stats.pct95,
            pct99: stats.pct99,
        }
    }
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
        let mut table = vec![];

        if let Some(ops_stats) = Stats::from_iter(results.iter().filter_map(|v| v.ops)) {
            table.push(StatsEntry::from_stats("ops", &ops_stats));
        }

        if let Some(duration_ns) = Stats::from_iter(results.iter().filter_map(|v| v.duration_ns)) {
            table.push(StatsEntry::from_stats("ns/op", &duration_ns));
        }

        if let Some(allocs_per_op) =
            Stats::from_iter(results.iter().filter_map(|v| v.allocs_per_op))
        {
            table.push(StatsEntry::from_stats("allocs/op", &allocs_per_op));
        }

        if let Some(bytes_per_op) = Stats::from_iter(results.iter().filter_map(|v| v.bytes_per_op))
        {
            table.push(StatsEntry::from_stats("B/op", &bytes_per_op));
        }

        println!("{name}\n{}", Table::new(table).with(Style::modern()));
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
