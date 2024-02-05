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
    io::{self, BufReader, Write},
    path::PathBuf,
};
use tabled::{settings::Style, Table, Tabled};

/// Statistically analyze Go benchmark output
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input files with Go benchmark outputs
    input: Vec<PathBuf>,

    /// Output results as CSV
    #[arg(short, long)]
    csv: bool,
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

    fn write_csv_header(mut w: impl io::Write) -> Result<()> {
        Ok(writeln!(
            w,
            "name,metric,min,max,avg,sd,median,pct90,pct95,pct99"
        )?)
    }

    fn write_csv(&self, name: &str, mut w: impl io::Write) -> Result<()> {
        Ok(writeln!(
            w,
            "{name},{},{},{},{},{},{},{},{},{}",
            self.metric,
            self.min,
            self.max,
            self.avg,
            self.sd,
            self.median,
            self.pct90,
            self.pct95,
            self.pct99
        )?)
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
    let mut result_entries = HashMap::new();

    for (name, results) in results {
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

        result_entries.insert(name, table);
    }

    if cli.csv {
        StatsEntry::write_csv_header(io::stdout())?;
        for (name, entries) in result_entries {
            for e in entries {
                e.write_csv(&name, io::stdout())?;
            }
        }
    } else {
        for (name, entries) in result_entries {
            writeln!(
                io::stdout(),
                "{name}\n{}\n",
                Table::new(entries).with(Style::modern())
            )?;
        }
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
