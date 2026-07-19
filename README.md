# rust-template

A GitHub template for starting a new Rust project on the [official Cargo package layout](https://doc.rust-lang.org/cargo/guide/project-layout.html), with a light Clean Architecture split for backend-style logic.

## Getting started

1. Click **Use this template** on GitHub (or `git clone` this repo).
2. Rename the crate:

   ```sh
   make rename NAME=my-cool-service
   ```

   This rewrites `rust-template` / `rust_template` in `Cargo.toml`, `README.md`, `src/main.rs`, and `tests/integration_test.rs`.
3. `make check`, start coding. Run `make help` to see all available commands.

## Layout

```
.
├── Cargo.toml
├── Makefile                  # build/test/lint/rename commands, see `make help`
├── src/
│   ├── main.rs                # composition root: wires config, logging, adapters
│   ├── lib.rs                 # public module tree
│   ├── config.rs              # env-based configuration
│   ├── error.rs                # crate-wide error type
│   ├── core/                  # entities + use cases — no dependency on api/utils
│   │   ├── domain.rs
│   │   └── usecase.rs
│   ├── api/                   # interface adapters (handlers, DTOs)
│   │   ├── mod.rs
│   │   └── dto.rs
│   └── utils/                 # frameworks & drivers (repository impls, logging setup)
└── tests/
    └── integration_test.rs
```

### Architecture

Dependencies point inward, per Clean Architecture:

- **`core`** — domain entities (`domain.rs`) and use cases (`usecase.rs`). Defines *ports* (traits like `ItemRepository`) that outer layers implement. Has zero dependency on `api` or `utils`.
- **`api`** — interface adapters. Translates between the outside world (CLI args, HTTP requests, ...) and use cases; maps domain types to DTOs.
- **`utils`** — frameworks & drivers. Concrete implementations of `core`'s ports (the template ships an in-memory `ItemRepository` — swap it for a real database adapter without touching `core` or `api`), plus cross-cutting helpers like `tracing` setup.
- **`main.rs`** — composition root. The only place that wires a concrete `utils` adapter into a `core` use case via `api`.

To add a real backend (e.g. `axum` + `sqlx`), add the framework to `Cargo.toml`, put the HTTP handlers in `api`, and put the database-backed repository in `utils` implementing the existing `ItemRepository`-style trait from `core`.

## Tooling

- **Makefile** — `make help` lists every command (`build`, `run`, `test`, `fmt`, `fmt-check`, `clippy`, `check`, `doc`, `clean`, `install-hooks`, `rename`).
- **Dependabot** — [.github/dependabot.yml](.github/dependabot.yml) checks `cargo` dependencies weekly and opens PRs.
- **pre-commit** — [.pre-commit-config.yaml](.pre-commit-config.yaml) runs `cargo fmt --check` and `cargo clippy` before each commit. Install with `pip install pre-commit && make install-hooks`.

CI is intentionally not included — add a workflow under `.github/workflows/` if/when the project needs one.
