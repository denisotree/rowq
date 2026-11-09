# rowq documentation

A fast, keyboard-driven terminal explorer for tabular data —
**CSV · JSON · JSONL · YAML · TOML · Parquet · Arrow · Excel · Markdown ·
SQLite · DuckDB**.

> 🇷🇺 Эта документация также доступна на [русском языке](../ru/README.md).

> rowq continues tuitab, which stays at 0.11.1 — see
> [Coming from tuitab](../../README.md#coming-from-tuitab) to move an existing install.

## Contents

| Guide | What's inside |
|-------|---------------|
| [Getting started](getting-started.md) | Install, open files, directory & multi-file browsing, stdin/pipe mode |
| [Keybindings](keybindings.md) | The complete command reference, grouped by mode |
| [Expressions](expressions.md) | Computed columns and expression filters: operators, functions, dates |
| [Charts](charts.md) | Histogram, frequency, line, and grouped-bar charts; pinning & drill-down |
| [JOIN](join.md) | The step-by-step JOIN wizard, join types, and DIFF of two tables |
| [Databases](database.md) | Editing a SQLite/DuckDB table and writing it back, and building one from nothing |
| [Pivot tables](pivot.md) | Pivot syntax, aggregations, and worked examples |
| [MCP server](mcp.md) | Letting an AI assistant compute with rowq's engine: tools, operations, connecting |
| [Recipes](recipes.md) | Task-oriented how-tos: dedup, % of total, clipboard, export, and more |

## At a glance

```sh
rowq data.csv                   # open a file
rowq config.toml                # JSON / YAML / TOML open as a browsable tree
rowq orders.csv customers.csv   # browse several files as a list
rowq ./reports/                 # browse a directory
rowq inventory.sqlite           # a file that is not there yet opens blank
cat data.csv | rowq -t csv      # read from a pipe
```

Once a table is open: move with `hjkl`, sort with `[` / `]` (`z[` / `z]` for a
second key), chart with `V`, get statistics with `I`, add a window column with
`zw`, and press `?` for in-app help. Full details in
[Keybindings](keybindings.md).

## See also

- [Main project README](https://github.com/denisotree/rowq/blob/master/README.md)
- [Contributing guide](https://github.com/denisotree/rowq/blob/master/CONTRIBUTING.md)
- [Changelog](https://github.com/denisotree/rowq/blob/master/CHANGELOG.md)
