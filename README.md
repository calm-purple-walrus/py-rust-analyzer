# py-rust-analyzer

Python bindings of [rust-analyzer](https://github.com/rust-lang/rust-analyzer)'s library crates (`ra_ap_*`), built
with [PyO3](https://pyo3.rs). Each binding exposes one function, method, field or enum variant. The Python module
is `rust_analyzer`.

## What it binds

Each version binds one release of rust-analyzer's crates and builds with the Rust toolchain that includes that
release of rust-analyzer.
`Cargo.toml` pins the crates with `=`; `rust-toolchain.toml` pins the toolchain.

| py-rust-analyzer | rust-analyzer crates (`ra_ap_*`) | Rust toolchain |
| ---------------- | -------------------------------- | -------------- |
| 0.1.0            | 0.0.352                          | 1.100.0        |
| 0.1.1            | 0.0.352                          | 1.100.0        |

## Versions

The version is independent of rust-analyzer's. rust-analyzer's crates are at 0.0.x and promise no compatibility
between releases, so:

- Moving to another rust-analyzer release is a new minor version (0.1 to 0.2) and a new row in the table.
- A change that breaks code using the bindings, such as a removed or renamed binding, is also a new minor version.
- New bindings and fixes that break no code using them are a new patch version (0.1.0 to 0.1.1).

Each version has a git tag (`v0.1.0`). Depend on a tag, not a branch.

## Adding a binding

Follow the rules in [AGENTS.md](AGENTS.md).

The types of the module are in the stub `rust_analyzer.pyi`, which is installed with it. Add each new binding to the
stub; `tests/test_stub.py` fails when the stub and the module differ.

## Build and test

The tests build the module with cargo and import it, so they run against the code in the checkout. To run the tests
and the checks:

```sh
uv sync
uv run pytest tests
uv run ruff check tests
uv run pyright
```

## License

Licensed under either the MIT license ([LICENSE-MIT](LICENSE-MIT)) or the Apache License 2.0
([LICENSE-APACHE](LICENSE-APACHE)), at your option.
