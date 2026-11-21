# py-rust-analyzer

Python bindings of [rust-analyzer](https://github.com/rust-lang/rust-analyzer)'s library crates (`ra_ap_*`), built
with [PyO3](https://pyo3.rs). Each binding exposes one item of rust-analyzer to Python under the same name: a function,
method, field or variant. The Python module is `rust_analyzer`.

## What it tracks

Each version binds exactly one release of rust-analyzer's crates and builds with the Rust toolchain that ships that
release. `Cargo.toml` pins the crates with `=`; `rust-toolchain.toml` pins the toolchain.

| py-rust-analyzer | rust-analyzer crates (`ra_ap_*`) | Rust toolchain |
| ---------------- | -------------------------------- | -------------- |
| 0.1.0            | 0.0.352                          | 1.100.0        |

## How the version changes

The version is this project's own; it does not copy rust-analyzer's. rust-analyzer's crates are at 0.0.x and promise
no compatibility between releases, so:

- Moving to another rust-analyzer release is a new minor version (0.1 to 0.2), with a new row in the table above.
- Any other change that breaks a caller, such as a binding removed or renamed, is a new minor version as well.
- Added bindings and fixes that break no caller are a new patch version (0.1.0 to 0.1.1).

Each version is a git tag, `v0.1.0` and so on. Depend on a tag, not on a branch.

## Build and test

The tests build the module with cargo and import it, so they always run against the code in the checkout:

```sh
uv sync
uv run pytest tests
```

Most tests load `tests/fixtures/harbor`, a small cargo workspace, and `tests/fixtures/built`, a crate with a build
script. Loading runs cargo in a copy of each.
