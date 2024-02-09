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

The result could look as following.

```
Strings
┌───────────┬────────┬────────┬──────────┬────────────────────┬────────┬────────┬────────┬────────┐
│ metric    │ min    │ max    │ avg      │ sd                 │ median │ pct90  │ pct95  │ pct99  │
├───────────┼────────┼────────┼──────────┼────────────────────┼────────┼────────┼────────┼────────┤
│ ops       │ 2828   │ 3499   │ 3152.2   │ 304.05223235490314 │ 2834   │ 3499   │ 3513   │ 3523   │
├───────────┼────────┼────────┼──────────┼────────────────────┼────────┼────────┼────────┼────────┤
│ ns/op     │ 342339 │ 358651 │ 349877.4 │ 4636.845073970016  │ 353727 │ 348526 │ 349467 │ 350219 │
├───────────┼────────┼────────┼──────────┼────────────────────┼────────┼────────┼────────┼────────┤
│ allocs/op │ 57     │ 70     │ 63.7     │ 6.116371473349211  │ 70     │ 57     │ 57     │ 57     │
├───────────┼────────┼────────┼──────────┼────────────────────┼────────┼────────┼────────┼────────┤
│ B/op      │ 1373   │ 1699   │ 1538.7   │ 148.48302933332144 │ 1695   │ 1373   │ 1368   │ 1363   │
└───────────┴────────┴────────┴──────────┴────────────────────┴────────┴────────┴────────┴────────┘

SimpleSum
┌───────────┬───────┬───────┬─────────┬───────────────────┬────────┬───────┬───────┬───────┐
│ metric    │ min   │ max   │ avg     │ sd                │ median │ pct90 │ pct95 │ pct99 │
├───────────┼───────┼───────┼─────────┼───────────────────┼────────┼───────┼───────┼───────┤
│ ops       │ 51964 │ 57012 │ 56210.1 │ 1459.628134149243 │ 56945  │ 51964 │ 49449 │ 47436 │
├───────────┼───────┼───────┼─────────┼───────────────────┼────────┼───────┼───────┼───────┤
│ ns/op     │ 20906 │ 23989 │ 21343.9 │ 884.0968781756895 │ 21014  │ 21025 │ 19543 │ 18357 │
├───────────┼───────┼───────┼─────────┼───────────────────┼────────┼───────┼───────┼───────┤
│ allocs/op │ 0     │ 0     │ 0       │ 0                 │ 0      │ 0     │ 0     │ 0     │
├───────────┼───────┼───────┼─────────┼───────────────────┼────────┼───────┼───────┼───────┤
│ B/op      │ 14    │ 15    │ 14.1    │ 0.3               │ 14     │ 15    │ 16    │ 16    │
└───────────┴───────┴───────┴─────────┴───────────────────┴────────┴───────┴───────┴───────┘
```

## Install

You can either download the latest release builds form the [Releases page](https://github.com/zekroTJA/testanalyzer/releases) or you can install it using cargo install.
```
cargo install --git https://github.com/zekroTJA/testanalyzer
```

## Credits

The output analyzer regular expression has been copied form [maragudk's](https://github.com/maragudk) [go-bench2csv tool](https://github.com/maragudk/go-bench2csv/blob/7b3cb2ca36e71970b5c8da1dadf67ea8b2b2b109/bench2csv.go#L24-L45), distributed under the [MIT License](https://github.com/maragudk/go-bench2csv/blob/main/LICENSE).
