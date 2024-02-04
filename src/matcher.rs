use anyhow::Result;
use regex::Regex;
use std::{
    io::{self},
    time::Duration,
};

// Source: https://github.com/maragudk/go-bench2csv/blob/7b3cb2ca36e71970b5c8da1dadf67ea8b2b2b109/bench2csv.go#L24-L45
const RX_LIT: &str = concat! {
    r"^Benchmark", // 'Benchmark' Prefix
    r"(?P<name>[^-\s]+)", // Name
    r"(?:-(?P<parallelism>\d+))?", // Optional parallelism (set with -cpu flag)
    r"\s+",
    r"(?P<ops>\d+)", // Operations run
    r"\s+",
    r"(?P<duration>\d+(?:\.\d+)?)", // Duration for each operation
    r"\sns/op",

    // Optionally, with -benchmem ...
    r"(\s+",
    r"(?P<bytesPerOp>\d+)", // Bytes per operation
    r"\sB/op\s+", // Bytes per operation unit suffix
    r"(?P<allocsPerOp>\d+)", // Allocs per operation
    r"\sallocs/op", // Allocs per operation unit suffix
    r")?",

    r"$",
};

#[derive(Clone, Debug)]
pub struct BenchResult {
    pub name: String,
    pub parallelism: Option<usize>,
    pub ops: Option<usize>,
    pub duration: Option<Duration>,
    pub bytes_per_op: Option<usize>,
    pub allocs_per_op: Option<usize>,
}

pub struct Matcher {
    rx: Regex,
}

impl Matcher {
    pub fn new() -> Matcher {
        let rx = Regex::new(RX_LIT).expect("Regex Initialization");
        Matcher { rx }
    }

    pub fn process_line(&self, line: &str) -> Result<Option<BenchResult>> {
        let Some(groups) = self.rx.captures(line) else {
            return Ok(None);
        };

        let name = groups
            .name("name")
            .ok_or_else(|| anyhow::anyhow!("no benchmark name found"))?
            .as_str()
            .into();

        let parallelism = groups
            .name("parallelism")
            .map(|v| v.as_str().parse())
            .transpose()?;

        let ops = groups.name("ops").map(|v| v.as_str().parse()).transpose()?;

        let duration = groups
            .name("duration")
            .map(|v| v.as_str().parse().map(Duration::from_nanos))
            .transpose()?;

        let bytes_per_op = groups
            .name("bytesPerOp")
            .map(|v| v.as_str().parse())
            .transpose()?;

        let allocs_per_op = groups
            .name("allocsPerOp")
            .map(|v| v.as_str().parse())
            .transpose()?;

        let res = BenchResult {
            name,
            parallelism,
            ops,
            duration,
            bytes_per_op,
            allocs_per_op,
        };

        Ok(Some(res))
    }

    pub fn process(&self, mut r: impl io::BufRead) -> Result<Vec<BenchResult>> {
        let mut s = String::new();
        let mut results = vec![];

        loop {
            s.clear();
            r.read_line(&mut s)?;
            if s.is_empty() {
                break;
            };

            if let Some(res) = self.process_line(s.trim())? {
                results.push(res);
            }
        }

        Ok(results)
    }
}
