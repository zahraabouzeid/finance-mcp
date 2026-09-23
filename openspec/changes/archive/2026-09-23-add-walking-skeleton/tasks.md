# Tasks

## 1. Skeleton

- [x] 1.1 Add the dependencies and dev dependencies from `design.md` to `Cargo.toml` and verify `cargo build` succeeds on the locked versions
- [x] 1.2 Create `src/lib.rs` and reduce `src/main.rs` to a call into the library, and verify `cargo build` and `cargo clippy --all-targets -- -D warnings` pass

## 2. Configuration

- [x] 2.1 Write unit tests for argument parsing: default port `8765`, `--port 9100`, and an invalid port that fails with a message naming the value; verify they fail
- [x] 2.2 Implement the `clap` argument struct with the `FINANCE_MCP_PORT` fallback and verify the tests from 2.1 pass

## 3. server_status tool

- [x] 3.1 Write unit tests for the `server_status` result: `status` is `ok`, `name` is `finance-mcp`, `version` matches `Cargo.toml`, and no other fields exist; verify they fail
- [x] 3.2 Implement the result type and the `server_status` tool with `rmcp` macros and verify the tests from 3.1 pass

## 4. HTTP server

- [x] 4.1 Write integration tests that start the server on port `0`: the bound address is `127.0.0.1`, an `rmcp` client initializes, lists `server_status` with no required parameters, and calls it successfully; verify they fail
- [x] 4.2 Write integration tests with raw HTTP requests: a foreign `Host` is rejected, a request with an `Origin` header is rejected, a request without `Origin` is processed; verify they fail
- [x] 4.3 Write an integration test that triggers the shutdown future and checks that the server stops; verify it fails
- [x] 4.4 Implement the library server: bind handling, `StreamableHttpService` in an `axum` router at `/mcp`, stateless JSON mode, explicit loopback `allowed_hosts`, enforced Origin validation and graceful shutdown; verify the tests from 4.1 to 4.3 pass

## 5. Binary

- [x] 5.1 Write binary tests that run the built executable: `FINANCE_MCP_PORT` sets the port, `--port` overrides it, a port already in use exits non-zero with a message naming the port, and the startup log goes to stderr with nothing on stdout; verify they fail
- [x] 5.2 Implement `main.rs`: parse arguments, set up `tracing` on stderr, bind with `anyhow` context, log the listening address, run the server with Ctrl+C as shutdown; verify the tests from 5.1 pass

## 6. Verification

- [x] 6.1 Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` and `cargo test --workspace --all-features --locked`, and verify all pass
- [x] 6.2 Start the server with `cargo run`, connect with an MCP client, call `server_status`, then press Ctrl+C, and verify the result and a clean exit with code 0
- [x] 6.3 Run `openspec validate add-walking-skeleton --strict` and verify it passes
