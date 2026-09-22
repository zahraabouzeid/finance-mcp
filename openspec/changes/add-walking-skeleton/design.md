# Design

## Context

The repository contains a single Cargo package with a placeholder `main.rs`. CI checks formatting, lints, tests on Linux and Windows, and `cargo deny`. See `proposal.md` for motivation and `specs/` for the required behavior.

Versions below were checked on crates.io on 2026-09-23.

## Goals / Non-Goals

**Goals:**
- A thin, testable server where the binary only parses arguments and starts the library.
- Every scenario in `specs/` is covered by an automated test, except a real Ctrl+C signal, which is verified manually.
- Security checks rely on the MCP SDK's built-in validation instead of custom middleware.

**Non-Goals:**
- Abstractions such as ports, traits for providers or dependency injection. They come with the architecture change.
- Stable module boundaries. Everything may move in the next change.

## Decisions

### 1. One package with a library and a binary target

`src/lib.rs` holds the server, `src/main.rs` only parses arguments, sets up logging and calls the library.

- Why: Integration tests in `tests/` can only import a library target, not a binary. A thin `main.rs` keeps almost all code testable.
- Alternative: Everything in `main.rs`. Rejected because it cannot be tested from `tests/`.
- Rust concept: A package can contain one library crate and several binary crates. The binary uses the library like any external crate, through its public items (`pub`).

### 2. MCP SDK: `rmcp` 3.4

The official Rust SDK from the Model Context Protocol organization. It implements the MCP specification `2026-07-28`.

- The tool is declared with the `#[tool_router]` and `#[tool]` attribute macros on an `impl` block. The server type implements the `ServerHandler` trait.
- The tool returns its result wrapped in `rmcp`'s `Json<T>`, so the result is sent as structured content with an output schema.
- The result type derives `Serialize` and `JsonSchema`. `JsonSchema` is taken from `rmcp`'s re-export of `schemars`, so both always use the same `schemars` version.
- Alternative: `rust-mcp-sdk`. Rejected because it is not the official SDK.
- Rust concepts: attribute and derive macros generate code at compile time. Traits define shared behavior, similar to an abstract base class, but without inheritance.

### 3. Streamable HTTP through `rmcp` and `axum`

`rmcp`'s `StreamableHttpService` is mounted into an `axum` router at `/mcp`.

- Stateless mode with JSON responses (`legacy_session_mode = false`, `json_response = true`). The `2026-07-28` protocol is stateless, so no session store is needed.
- Alternative: Session mode with an in-memory session manager. Rejected because it adds state that the new protocol does not need.
- Rust concept: `axum` and `rmcp` both build on the `tower::Service` trait, which is why one can be plugged into the other.

### 4. Host and Origin validation

Both checks use `StreamableHttpServerConfig`:

- Host: the default `allowed_hosts` accepts loopback hosts only. The code sets it explicitly anyway, so the intent is visible and a future change to the SDK default cannot weaken it.
- Origin: `enforce_origin_validation()` with an empty allow list. Every request that carries an `Origin` header is rejected. Native MCP clients do not send `Origin`, so they are not affected.
- Alternative: Custom `tower` middleware. Rejected because the SDK already implements both checks as the MCP specification requires.

### 5. Binding

The server binds a `tokio::net::TcpListener` to `127.0.0.1` and the configured port. The library takes the bound listener as input, so tests can bind port `0`, let the operating system pick a free port and read it back.

- Rust concept: `std::net::Ipv4Addr::LOCALHOST` is a constant. Using a typed address instead of the string `"127.0.0.1"` rules out typos at compile time.

### 6. Configuration with `clap`

A struct with `#[derive(Parser)]` defines the arguments. The port field is a `u16` with `#[arg(long, env = "FINANCE_MCP_PORT", default_value_t = 8765)]`.

- `clap` applies the precedence argument, then environment variable, then default.
- Invalid values fail during parsing with a message that names the value. A `u16` cannot hold values above 65535, so no manual range check is needed.
- Alternative: Reading `std::env::var` by hand. Rejected because it duplicates what `clap` does and needs manual parsing and error messages.
- Rust concept: "Parse, don't validate". Once parsing succeeds, the type guarantees the value is valid.

### 7. Async runtime: `tokio`

`#[tokio::main]` starts the multi-threaded runtime. `rmcp` and `axum` both require `tokio`.

- Rust concept: `async fn` returns a future that does nothing until it is awaited. The runtime drives the futures. This is similar to Python's `asyncio`, but the runtime is a library you choose, not part of the language.

### 8. Errors

- The library returns `std::io::Result`, because binding and serving fail with I/O errors.
- The binary uses `anyhow` and adds context, for example `failed to bind 127.0.0.1:8765`, so the error names the port.
- Rust concept: The `?` operator returns early with the error. `anyhow::Context` wraps an error with a readable message, similar to `raise ... from ...` in Python.

### 9. Logging

`tracing` with `tracing-subscriber`, writing to standard error. The startup message logs the listening address. `rmcp` already emits `tracing` events, so they appear in the same output.

- Alternative: `eprintln!`. Rejected because the SDK's own logs would not be visible.

### 10. Graceful shutdown

The library takes a shutdown future as input. `main.rs` passes `tokio::signal::ctrl_c()`, tests pass a future they control. On shutdown, `axum` stops accepting connections and `rmcp`'s cancellation token ends in-flight work.

### 11. Server name and version

`name` and `version` come from `env!("CARGO_PKG_NAME")` and `env!("CARGO_PKG_VERSION")`.

- Rust concept: `env!` reads values at compile time. The version in the response always matches `Cargo.toml` with no runtime lookup.

### 12. Testing strategy

Tests are written before the implementation where possible.

| Scenario group | Test type | How |
|---|---|---|
| `server_status` result | unit test | call the tool method directly |
| Default port, `--port`, invalid port | unit test | `Args::try_parse_from` with argument lists |
| Environment variable and precedence | binary test | start the built binary with `std::process::Command` and `.env(...)` |
| Port in use, stderr only | binary test | bind a port in the test, start the binary, check exit code and output |
| Connect, list tools, call tool | integration test | start the server on port `0`, connect with the `rmcp` client |
| Host and Origin rejection | integration test | send raw HTTP requests with custom headers |
| Loopback only | integration test | check that the bound address is `127.0.0.1` |
| Graceful shutdown | integration test + manual | trigger the shutdown future in tests, press Ctrl+C manually once |

- Environment variables are set only for child processes. Setting them in the test process with `std::env::set_var` is `unsafe` since Rust 2024, because other threads may read them at the same time.
- `env!("CARGO_BIN_EXE_finance-mcp")` gives integration tests the path of the built binary.

### Dependencies

| Crate | Version | Features | Reason |
|---|---|---|---|
| `rmcp` | 3.4 | `server`, `macros`, `schemars`, `transport-streamable-http-server` | MCP protocol, tool macros, HTTP transport |
| `tokio` | 1.53 | `rt-multi-thread`, `macros`, `net`, `signal` | async runtime, TCP listener, Ctrl+C |
| `axum` | 0.8 | default | HTTP router that hosts the MCP service |
| `clap` | 4.6 | `derive`, `env` | argument and environment variable parsing |
| `serde` | 1 | `derive` | serialize the tool result |
| `anyhow` | 1 | default | error context in the binary |
| `tracing` | 0.1 | default | logging API |
| `tracing-subscriber` | 0.3 | `fmt` | log output to stderr |

Dev dependencies:

| Crate | Features | Reason |
|---|---|---|
| `rmcp` | `client`, `transport-streamable-http-client-reqwest`, without TLS features | MCP client for integration tests |
| `reqwest` | no default features | raw HTTP requests with custom headers |

Tests only call `http://127.0.0.1`, so no TLS library is needed. If `CancellationToken` is not re-exported by `rmcp`, `tokio-util` is added for it.

## Risks / Trade-offs

- [`rmcp` 3.x changes quickly] → `Cargo.lock` pins exact versions. Read the changelog before upgrading.
- [Origin enforcement blocks browser-based MCP tools, such as a web inspector that connects directly] → Use clients that connect without a browser. Allowing specific origins can be added in a later change.
- [Clients that resolve `localhost` to IPv6 `::1` cannot connect, because the server binds IPv4 only] → Document the URL `http://127.0.0.1:<port>/mcp`.
- [Older clients that expect session mode] → The integration tests use the current `rmcp` client. Compatibility with older protocol versions is not a goal of this change.
- [A dependency pulls in a TLS library with native build requirements on Windows] → No TLS features are enabled. CI on Windows and `cargo deny` catch it.
- [Default port `8765` is taken on a machine] → Configurable through `--port` and `FINANCE_MCP_PORT`.

## Migration Plan

No migration. The placeholder program is replaced. Rollback is a revert of the pull request.
