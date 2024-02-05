# testanalyzer

A CLI tool to statistically analyze the outputs of the [Go benchmark tool](https://golangdocs.com/benchmark-functions-in-golang).

## Usage

```
$ testanalyzer --help
Statistically analyze Go benchmark output

Usage: testanalyzer.exe [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...  Input files with Go benchmark outputs

Options:
  -c, --csv      Output results as CSV
  -h, --help     Print help
  -V, --version  Print version
```

You can chain this tool directly after `go test` to analyze the output.

```
go test -bench=. -benchmem -count=100 | testanalyzer
```

## Install

You can either download the latest release builds form the [Releases page](https://github.com/zekroTJA/testanalyzer/releases) or you can install it using cargo install.
```
cargo install --git https://github.com/zekroTJA/testanalyzer
```