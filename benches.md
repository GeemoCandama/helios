# Helios Benchmarking

Helios performance is measured using [criterion](https://github.com/bheisler/criterion.rs) for comprehensive statistics-driven benchmarking.

Benchmarks are defined in the [benches](./benches/) subdirectory and can be run using the cargo `bench` subcommand (eg `cargo bench`). To run a specific benchmark, you can use `cargo bench --bench <name>`, where `<name>` is one of the benchmarks defined in the [Cargo.toml](./Cargo.toml) file under a `[[bench]]` section.



#### Flamegraphs

[Flamegraph](https://github.com/brendangregg/FlameGraph) is a powerful rust crate for generating profile visualizations, that is graphing the time a program spends in each function. Functions called during execution are displayed as horizontal rectangles with the width proportional to the time spent in that function. As the call stack grows (think nested function invocations), the rectangles are stacked vertically. This provides a powerful visualization for quickly understanding which parts of a codebase take up disproportionate amounts of time.

To generate a flamegraph, you can use the `cargo flamegraph` subcommand. For example, to generate a flamegraph for the [`client`](./examples/client.rs) example, you can run:

```bash
cargo flamegraph --example client -o flamegraph.svg
```

Helios stores generated flamegraphs in the [flamegraphs](./flamegraphs/) directory, with [benches](./flamegraphs/benches/) and [examples](./flamegraphs/examples/) subdirectories containing their associated flamegraphs.

To generate and save a flamegraph for a given example, you can use the `-o` flag to specify the output path. For example,

```bash
cargo flamegraph --example client -o flamegraphs/examples/client.svg
```