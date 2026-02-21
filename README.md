# img-scraper

A small Rust workspace for experimenting with crate boundaries and tracing-based observability.

## Workspace layout

- `img-scraper` (root package): default binary scaffold currently printing `Hello, world!`.
- `api`: example application entrypoint that configures logging and calls into the domain layer.
- `domain`: business logic (`add`) instrumented with `tracing`.
- `logging`: shared logging bootstrap using `tracing-subscriber`.

## Prerequisites

- Rust toolchain (stable)
- Cargo

## Build

```bash
cargo build
```

## Run

Run the API binary from the workspace root:

```bash
cargo run -p api
```

Expected output includes a tracing span/event and:

```text
The answer is: 109
```

You can also run the root scaffold binary:

```bash
cargo run -p img-scraper
```

## Test

Run all workspace tests:

```bash
cargo test
```

## Notes

- The `api` crate demonstrates composition of `domain` and `logging`.
- `domain::add` is instrumented with `#[tracing::instrument]` and emits an info-level event.
- `logging::setup_logging` enables formatted tracing output and active span lifecycle events.
